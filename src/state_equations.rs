use anyhow::{anyhow, bail, Context, Result};
use na::{SMatrix, SVector};

type Matrix = SMatrix<f64, 10, 10>;
type Vector = SVector<f64, 10>;

// Specify how many times the flow directions can be adjusted before failing
const MAX_FLOW_ADJUSTMENTS: usize = 3;

/// Solve the state equations
///
/// TODO: Add documentation here about `Ax=b`, the flow direction / enthalpy relationship / etc.
/// This function is generic over the decomposition function used to solve `Ax=b`.
pub fn solve<T: MatrixDecomposition>(
    inputs: Inputs,
    flow_dir_hint: FlowDirection,
) -> Result<Solution> {
    let system = System::new(inputs);
    let mut flow_dir = flow_dir_hint;
    for _ in 0..=MAX_FLOW_ADJUSTMENTS {
        let solution = system.solve::<T>(flow_dir)?;
        let actual_flow_dir = FlowDirection {
            ck: Direction::from_value(solution.m_dot_ck),
            kr: Direction::from_value(solution.m_dot_kr),
            rl: Direction::from_value(solution.m_dot_rl),
            le: Direction::from_value(solution.m_dot_le),
        };
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
        a[(1, 1)] = 1.0; // m_dot_ck
        a[(1, 8)] = comp.vol * comp.dd_dT_P; // dTc_dt
        a[(1, 10)] = comp.vol * comp.dd_dP_T; // dP_dt
        b[(1)] = -comp.dens * comp.dV_dt;

        // Energy balance on compression space
        // a[(2, 1)] = h_ck_norm; // m_dot_ck
        a[(2, 8)] = comp.vol * (comp.dens * comp.du_dT_P + comp.inte * comp.dd_dT_P) / enth_norm; // dTc_dt
        a[(2, 10)] = comp.vol * (comp.dens * comp.du_dP_T + comp.inte * comp.dd_dP_T) / enth_norm; // dP_dt
        b[(2)] = (-(pres + comp.dens * comp.inte) * comp.dV_dt - comp.Q_dot) / enth_norm;

        // Mass balance on cold heat exchanger
        a[(3, 1)] = -1.0; // m_dot_ck
        a[(3, 2)] = 1.0; // m_dot_kr
        a[(3, 10)] = chx.vol * chx.dd_dP_T; // dP_dt

        // Energy balance on cold heat exchanger
        // a[(4, 1)] = -h_ck_norm; // m_dot_ck
        // a[(4, 2)] = h_kr_norm; // m_dot_kr
        a[(4, 5)] = 1.0 / enth_norm; // Q_dot_k
        a[(4, 10)] = chx.vol * (chx.dens * chx.du_dP_T + chx.inte * chx.dd_dP_T) / enth_norm; // dP_dt

        // Mass balance on regenerator
        a[(5, 2)] = -1.0; // m_dot_kr
        a[(5, 3)] = 1.0; // m_dot_rl
        a[(5, 10)] = regen.vol * regen.dd_dP_T; // dP_dt

        // Energy balance on regenerator
        // a[(6, 2)] = -h_kr_norm; // m_dot_kr
        // a[(6, 3)] = h_rl_norm; // m_dot_rl
        a[(6, 6)] = 1.0 / enth_norm; // Q_dot_r
        a[(6, 10)] =
            regen.vol * (regen.dens * regen.du_dP_T + regen.inte * regen.dd_dP_T) / enth_norm; // dP_dt

        // Mass balance on hot heat exchanger
        a[(7, 3)] = -1.0; // m_dot_rl
        a[(7, 4)] = 1.0; // m_dot_le
        a[(7, 10)] = hhx.vol * hhx.dd_dP_T; // dP_dt

        // Energy balance on hot heat exchanger
        // a[(8, 3)] = -h_rl_norm; // m_dot_rl
        // a[(8, 4)] = h_le_norm; // m_dot_le
        a[(8, 7)] = -1.0 / enth_norm; // Q_dot_l
        a[(8, 10)] = hhx.vol * (hhx.dens * hhx.du_dP_T + hhx.inte * hhx.dd_dP_T) / enth_norm; // dP_dt

        // Mass balance on expansion space
        a[(9, 4)] = -1.0; // m_dot_le
        a[(9, 9)] = exp.vol * exp.dd_dT_P; // dTe_dt
        a[(9, 10)] = exp.vol * exp.dd_dP_T; // dP_dt
        b[(9)] = -exp.dens * exp.dV_dt;

        // Energy balance on expansion space
        // a[(10, 4)] = -h_le_norm; // m_dot_le
        a[(10, 9)] = exp.vol * (exp.dens * exp.du_dT_P + exp.inte * exp.dd_dT_P) / enth_norm; // dTe_dt
        a[(10, 10)] = exp.vol * (exp.dens * exp.du_dP_T + exp.inte * exp.dd_dP_T) / enth_norm; // dP_dt
        b[(10)] = (-(pres + exp.dens * exp.inte) * exp.dV_dt - exp.Q_dot) / enth_norm;

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

pub struct SVD;
impl MatrixDecomposition for SVD {
    fn solve(a: Matrix, b: &Vector) -> Result<Vector> {
        let eps = 1e-9; // TODO: look into what value makes sense here
        a.svd_unordered(true, true)
            .solve(b, eps)
            .map_err(|_| anyhow!("unable to solve matrix with SVD decomposition"))
    }
}

/// A thing that can generate the `A` matrix
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
        a[(2, 1)] = h_ck_norm;
        a[(4, 1)] = -h_ck_norm;

        a[(4, 2)] = h_kr_norm;
        a[(6, 2)] = -h_kr_norm;

        a[(6, 3)] = h_rl_norm;
        a[(8, 3)] = -h_rl_norm;

        a[(8, 4)] = h_le_norm;
        a[(10, 4)] = -h_le_norm;

        a
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Positive,
    Negative,
    Unknown,
}

impl Direction {
    /// Return a `Direction` based on the sign of a number
    ///
    /// If `value` is exactly `0.0`, a positive direction is assumed.
    fn from_value(value: f64) -> Self {
        if value >= 0.0 {
            Self::Positive
        } else {
            Self::Negative
        }
    }

    /// Return a value based on the direction of `self`
    ///
    /// An average of the two values is returned if the direction is `Unknown`.
    fn select(&self, positive: f64, negative: f64) -> f64 {
        match self {
            Self::Positive => positive,
            Self::Negative => negative,
            Self::Unknown => 0.5 * (positive + negative),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct FlowDirection {
    ck: Direction,
    kr: Direction,
    rl: Direction,
    le: Direction,
}

impl Default for FlowDirection {
    fn default() -> Self {
        Self {
            ck: Direction::Unknown,
            kr: Direction::Unknown,
            rl: Direction::Unknown,
            le: Direction::Unknown,
        }
    }
}

pub struct Inputs {
    pres: f64,
    enth_norm: f64,
    comp: WorkingSpaceInputs,
    chx: HeatExchangerInputs,
    regen: RegeneratorInputs,
    hhx: HeatExchangerInputs,
    exp: WorkingSpaceInputs,
}

#[allow(non_snake_case)]
pub struct WorkingSpaceInputs {
    vol: f64,
    dens: f64,
    inte: f64,
    enth: f64,
    dd_dP_T: f64,
    dd_dT_P: f64,
    du_dP_T: f64,
    du_dT_P: f64,
    dV_dt: f64,
    Q_dot: f64,
}

#[allow(non_snake_case)]
pub struct HeatExchangerInputs {
    vol: f64,
    dens: f64,
    inte: f64,
    enth: f64,
    dd_dP_T: f64,
    du_dP_T: f64,
}

#[allow(non_snake_case)]
pub struct RegeneratorInputs {
    vol: f64,
    dens: f64,
    inte: f64,
    enth_cold: f64,
    enth_hot: f64,
    dd_dP_T: f64,
    du_dP_T: f64,
}

#[allow(non_snake_case)]
pub struct Solution {
    pub m_dot_ck: f64,
    pub m_dot_kr: f64,
    pub m_dot_rl: f64,
    pub m_dot_le: f64,
    pub Q_dot_k: f64,
    pub Q_dot_r: f64,
    pub Q_dot_l: f64,
    pub dTc_dt: f64,
    pub dTe_dt: f64,
    pub dP_dt: f64,
}
