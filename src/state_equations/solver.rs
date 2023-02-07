use anyhow::{anyhow, bail, Context, Result};
use na::{SMatrix, SVector};

use super::types::{FlowDirection, Inputs, Solution};

type Matrix = SMatrix<f64, 10, 10>;
type Vector = SVector<f64, 10>;

// The maximum number of times flow directions can be updated before failing
const ALLOWED_FLOW_UPDATES: usize = 3;

/// Solve the state equations
///
/// This function is generic over the decomposition function used to solve `Ax=b`.
///
/// Better documentation will be added (https://github.com/isentropic-dev/sett-rs/issues/9)
pub(super) fn solve<T: MatrixDecomposition>(
    inputs: Inputs,
    flow_dir_hint: FlowDirection,
) -> Result<Solution> {
    let system = System::new(inputs);
    let mut flow_dir = flow_dir_hint;
    for _ in 0..=ALLOWED_FLOW_UPDATES {
        let solution = system.solve::<T>(flow_dir)?;
        let actual_flow_dir = FlowDirection::from_solution(&solution);
        if flow_dir == actual_flow_dir {
            return Ok(solution);
        }
        flow_dir = actual_flow_dir;
    }
    bail!("unable to determine flow directions")
}

/// Represents the `Ax=b` system of state equations
///
/// The `A` matrix depends on the direction of fluid flow between control
/// volumes, which are adjusted iteratively when solving the state equations.
/// The stored `MatrixStencil` is responsible for generating this matrix as a
/// function of flow directions.
struct System {
    stencil: MatrixStencil,
    b: Vector,
}

impl System {
    /// Create a new `System`
    fn new(inputs: Inputs) -> Self {
        let Inputs {
            pres,
            enth_norm,
            comp,
            chx,
            regen,
            hhx,
            exp,
        } = inputs;

        let mut a = Matrix::zeros();
        let mut b = Vector::zeros();

        // Enthalpies are normalized to reduce the matrix condition number
        let h_comp_norm = comp.enth / enth_norm;
        let h_chx_norm = chx.enth / enth_norm;
        let h_regen_cold_norm = regen.enth_cold / enth_norm;
        let h_regen_hot_norm = regen.enth_hot / enth_norm;
        let h_hhx_norm = hhx.enth / enth_norm;
        let h_exp_norm = exp.enth / enth_norm;

        // Mass balance on compression space
        a[(0, 0)] = 1.0; // m_dot_ck
        a[(0, 7)] = comp.vol * comp.dd_dT_P; // dTc_dt
        a[(0, 9)] = comp.vol * comp.dd_dP_T; // dP_dt
        b[(0)] = -comp.dens * comp.dV_dt;

        // Energy balance on compression space
        // a[(1, 0)] = h_ck_norm; // m_dot_ck (value is set by the `MatrixStencil` based on flow direction)
        a[(1, 7)] = comp.vol * (comp.dens * comp.du_dT_P + comp.inte * comp.dd_dT_P) / enth_norm; // dTc_dt
        a[(1, 9)] = comp.vol * (comp.dens * comp.du_dP_T + comp.inte * comp.dd_dP_T) / enth_norm; // dP_dt
        b[(1)] = (-(pres + comp.dens * comp.inte) * comp.dV_dt - comp.Q_dot) / enth_norm;

        // Mass balance on cold heat exchanger
        a[(2, 0)] = -1.0; // m_dot_ck
        a[(2, 1)] = 1.0; // m_dot_kr
        a[(2, 9)] = chx.vol * chx.dd_dP_T; // dP_dt

        // Energy balance on cold heat exchanger
        // a[(3, 0)] = -h_ck_norm; // m_dot_ck
        // a[(3, 1)] = h_kr_norm; // m_dot_kr
        a[(3, 4)] = 1.0 / enth_norm; // Q_dot_k
        a[(3, 9)] = chx.vol * (chx.dens * chx.du_dP_T + chx.inte * chx.dd_dP_T) / enth_norm; // dP_dt

        // Mass balance on regenerator
        a[(4, 1)] = -1.0; // m_dot_kr
        a[(4, 2)] = 1.0; // m_dot_rl
        a[(4, 9)] = regen.vol * regen.dd_dP_T; // dP_dt

        // Energy balance on regenerator
        // a[(5, 1)] = -h_kr_norm; // m_dot_kr
        // a[(5, 2)] = h_rl_norm; // m_dot_rl
        a[(5, 5)] = 1.0 / enth_norm; // Q_dot_r
        a[(5, 9)] =
            regen.vol * (regen.dens * regen.du_dP_T + regen.inte * regen.dd_dP_T) / enth_norm; // dP_dt

        // Mass balance on hot heat exchanger
        a[(6, 2)] = -1.0; // m_dot_rl
        a[(6, 3)] = 1.0; // m_dot_le
        a[(6, 9)] = hhx.vol * hhx.dd_dP_T; // dP_dt

        // Energy balance on hot heat exchanger
        // a[(7, 2)] = -h_rl_norm; // m_dot_rl
        // a[(7, 3)] = h_le_norm; // m_dot_le
        a[(7, 6)] = -1.0 / enth_norm; // Q_dot_l
        a[(7, 9)] = hhx.vol * (hhx.dens * hhx.du_dP_T + hhx.inte * hhx.dd_dP_T) / enth_norm; // dP_dt

        // Mass balance on expansion space
        a[(8, 3)] = -1.0; // m_dot_le
        a[(8, 8)] = exp.vol * exp.dd_dT_P; // dTe_dt
        a[(8, 9)] = exp.vol * exp.dd_dP_T; // dP_dt
        b[(8)] = -exp.dens * exp.dV_dt;

        // Energy balance on expansion space
        // a[(9, 3)] = -h_le_norm; // m_dot_le
        a[(9, 8)] = exp.vol * (exp.dens * exp.du_dT_P + exp.inte * exp.dd_dT_P) / enth_norm; // dTe_dt
        a[(9, 9)] = exp.vol * (exp.dens * exp.du_dP_T + exp.inte * exp.dd_dP_T) / enth_norm; // dP_dt
        b[(9)] = (-(pres + exp.dens * exp.inte) * exp.dV_dt - exp.Q_dot) / enth_norm;

        Self {
            stencil: MatrixStencil {
                a,
                h_comp_norm,
                h_chx_norm,
                h_regen_cold_norm,
                h_regen_hot_norm,
                h_hhx_norm,
                h_exp_norm,
            },
            b,
        }
    }

    /// Solve the system of equations
    fn solve<T: MatrixDecomposition>(&self, flow_dir: FlowDirection) -> Result<Solution> {
        let a = self.stencil.create_matrix(flow_dir);
        let x = T::solve(a, &self.b)?;
        Ok(Solution {
            m_dot_ck: x[0],
            m_dot_kr: x[1],
            m_dot_rl: x[2],
            m_dot_le: x[3],
            Q_dot_k: x[4],
            Q_dot_r: x[5],
            Q_dot_l: x[6],
            dTc_dt: x[7],
            dTe_dt: x[8],
            dP_dt: x[9],
        })
    }
}

struct MatrixStencil {
    a: Matrix,
    h_comp_norm: f64,
    h_chx_norm: f64,
    h_regen_cold_norm: f64,
    h_regen_hot_norm: f64,
    h_hhx_norm: f64,
    h_exp_norm: f64,
}

impl MatrixStencil {
    /// Create an `A` matrix based on the provided flow directions
    fn create_matrix(&self, flow_dir: FlowDirection) -> Matrix {
        // Determine enthalpies at the volume interfaces
        let h_ck_norm = flow_dir.ck.select(self.h_comp_norm, self.h_chx_norm);
        let h_kr_norm = flow_dir.kr.select(self.h_chx_norm, self.h_regen_cold_norm);
        let h_rl_norm = flow_dir.rl.select(self.h_regen_hot_norm, self.h_hhx_norm);
        let h_le_norm = flow_dir.le.select(self.h_hhx_norm, self.h_exp_norm);

        // Set the enthalpy entries in the matrix
        let mut a = self.a; // `SMatrix` is copy
        a[(1, 0)] = h_ck_norm;
        a[(3, 0)] = -h_ck_norm;

        a[(3, 1)] = h_kr_norm;
        a[(5, 1)] = -h_kr_norm;

        a[(5, 2)] = h_rl_norm;
        a[(7, 2)] = -h_rl_norm;

        a[(7, 3)] = h_le_norm;
        a[(9, 3)] = -h_le_norm;

        a
    }
}

pub trait MatrixDecomposition {
    fn solve(a: Matrix, b: &Vector) -> Result<Vector>;
}

pub struct QR;
impl MatrixDecomposition for QR {
    fn solve(a: Matrix, b: &Vector) -> Result<Vector> {
        a.qr()
            .solve(b)
            .context("unable to solve matrix with QR decompositon")
    }
}

pub struct LU;
impl MatrixDecomposition for LU {
    fn solve(a: Matrix, b: &Vector) -> Result<Vector> {
        a.lu()
            .solve(b)
            .context("unable to solve matrix with LU decomposition")
    }
}

pub struct Cholesky;
impl MatrixDecomposition for Cholesky {
    fn solve(a: Matrix, b: &Vector) -> Result<Vector> {
        let decomp = a
            .cholesky()
            .context("unable to solve matrix with Cholesky decomposition")?;
        let x = decomp.solve(b);
        Ok(x)
    }
}

pub struct SvdDefault;
impl MatrixDecomposition for SvdDefault {
    fn solve(a: Matrix, b: &Vector) -> Result<Vector> {
        let eps = 1e-12;
        a.svd_unordered(true, true)
            .solve(b, eps)
            .map_err(|_| anyhow!("unable to solve matrix with SVD decomposition"))
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::read_test_inputs;
    use super::*;

    #[test]
    fn test_typical_ideal_gas_hydrogen_values() {
        let inputs = read_test_inputs("ideal_gas_hydrogen.json");
        let inputs: Inputs = serde_json::from_str(&inputs).expect("test inputs file is invalid");
        let flow_dir = FlowDirection::default();

        let lu_solution = solve::<LU>(inputs, flow_dir).expect("should solve");
        insta::assert_yaml_snapshot!(lu_solution, @r###"
        ---
        m_dot_ck: -0.0369671135868011
        m_dot_kr: -0.04739861686084307
        m_dot_rl: -0.06366524032668251
        m_dot_le: -0.07450887512267486
        Q_dot_k: 11687.524503992354
        Q_dot_r: 436791.3029835215
        Q_dot_l: 83208.74490054866
        dTc_dt: 2845.6263552639434
        dTe_dt: 31166.869984699082
        dP_dt: 390423950.31296676
        "###);

        let qr_solution = solve::<QR>(inputs, flow_dir).expect("should solve");
        insta::assert_yaml_snapshot!(qr_solution, @r###"
        ---
        m_dot_ck: -0.03696711358680108
        m_dot_kr: -0.04739861686084307
        m_dot_rl: -0.06366524032668251
        m_dot_le: -0.07450887512267484
        Q_dot_k: 11687.524503992556
        Q_dot_r: 436791.3029835213
        Q_dot_l: 83208.74490054866
        dTc_dt: 2845.6263552639507
        dTe_dt: 31166.869984699046
        dP_dt: 390423950.3129669
        "###);

        let svd_solution = solve::<SvdDefault>(inputs, flow_dir).expect("should solve");
        insta::assert_yaml_snapshot!(svd_solution, @r###"
        ---
        m_dot_ck: -0.03696711358657141
        m_dot_kr: -0.047398616860491946
        m_dot_rl: -0.06366524032648578
        m_dot_le: -0.07450887512248028
        Q_dot_k: 11687.524503991895
        Q_dot_r: 436791.3029835225
        Q_dot_l: 83208.74490054886
        dTc_dt: 2845.626355263992
        dTe_dt: 31166.86998469886
        dP_dt: 390423950.3129672
        "###);
    }
    #[test]
    fn test_typical_refprop_hydrogen_values() {
        let inputs = read_test_inputs("refprop_hydrogen.json");
        let inputs: Inputs = serde_json::from_str(&inputs).expect("test inputs file is invalid");
        let flow_dir = FlowDirection::default();

        let lu_solution = solve::<LU>(inputs, flow_dir).expect("should solve");
        insta::assert_yaml_snapshot!(lu_solution, @r###"
        ---
        m_dot_ck: -0.028538301905757048
        m_dot_kr: -0.038613913985699036
        m_dot_rl: -0.05488487354380802
        m_dot_le: -0.06597724838021778
        Q_dot_k: 21675.379440630473
        Q_dot_r: 392576.6689847019
        Q_dot_l: 60769.16953666401
        dTc_dt: 4548.878957390213
        dTe_dt: 8995.893667930639
        dP_dt: 417493124.93544215
        "###);

        let qr_solution = solve::<QR>(inputs, flow_dir).expect("should solve");
        insta::assert_yaml_snapshot!(qr_solution, @r###"
        ---
        m_dot_ck: -0.028538301905757044
        m_dot_kr: -0.038613913985699
        m_dot_rl: -0.054884873543808
        m_dot_le: -0.06597724838021779
        Q_dot_k: 21675.37944063022
        Q_dot_r: 392576.66898470203
        Q_dot_l: 60769.169536663976
        dTc_dt: 4548.878957390214
        dTe_dt: 8995.893667930726
        dP_dt: 417493124.9354423
        "###);

        let svd_solution = solve::<SvdDefault>(inputs, flow_dir).expect("should solve");
        insta::assert_yaml_snapshot!(svd_solution, @r###"
        ---
        m_dot_ck: -0.02853830190560719
        m_dot_kr: -0.038613913985456536
        m_dot_rl: -0.05488487354363928
        m_dot_le: -0.06597724838007397
        Q_dot_k: 21675.379440630168
        Q_dot_r: 392576.6689847028
        Q_dot_l: 60769.16953666335
        dTc_dt: 4548.878957390116
        dTe_dt: 8995.893667931086
        dP_dt: 417493124.9354424
        "###);
    }
}
