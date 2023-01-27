use na::{SMatrix, SVector};

type Matrix10x10 = SMatrix<f64, 10, 10>;
type Vector10 = SVector<f64, 10>;

struct Equations {
    a: Matrix10x10,
    b: Vector10,
    h_comp_norm: f64,
    h_chx_norm: f64,
    h_regen_cold_norm: f64,
    h_regen_hot_norm: f64,
    h_hhx_norm: f64,
    h_exp_norm: f64,
}

impl Equations {
    fn new(inputs: EquationsInputs) -> Self {
        let EquationsInputs {
            pres,
            enth_norm,
            flow_dir,
            comp,
            chx,
            regen,
            hhx,
            exp,
        } = inputs;

        let mut a = Matrix10x10::zeros();
        let mut b = Vector10::zeros();

        // Enthalpies are normalized to reduce the matrix condition number
        let h_comp_norm = comp.enth / enth_norm;
        let h_chx_norm = comp.enth / enth_norm;
        let h_regen_cold_norm = comp.enth / enth_norm;
        let h_regen_hot_norm = comp.enth / enth_norm;
        let h_hhx_norm = comp.enth / enth_norm;
        let h_exp_norm = comp.enth / enth_norm;

        // The enthalpies at the volume interfaces depend on mass flow direction
        let h_ck_norm = flow_dir.ck.select(h_comp_norm, h_chx_norm);
        let h_kr_norm = flow_dir.kr.select(h_chx_norm, h_regen_cold_norm);
        let h_rl_norm = flow_dir.rl.select(h_regen_hot_norm, h_hhx_norm);
        let h_le_norm = flow_dir.le.select(h_hhx_norm, h_exp_norm);

        // Mass balance on compression space
        a[(1, 1)] = 1.0; // m_dot_ck
        a[(1, 8)] = comp.vol * comp.dd_dT_P; // dTc_dt
        a[(1, 10)] = comp.vol * comp.dd_dP_T; // dP_dt
        b[(1)] = -comp.dens * comp.dV_dt;

        // Energy balance on compression space
        a[(2, 1)] = h_ck_norm; // m_dot_ck
        a[(2, 8)] = comp.vol * (comp.dens * comp.du_dT_P + comp.inte * comp.dd_dT_P) / enth_norm; // dTc_dt
        a[(2, 10)] = comp.vol * (comp.dens * comp.du_dP_T + comp.inte * comp.dd_dP_T) / enth_norm; // dP_dt
        b[(2)] = (-(pres + comp.dens * comp.inte) * comp.dV_dt - comp.Q_dot) / enth_norm;

        // Mass balance on cold heat exchanger
        a[(3, 1)] = -1.0; // m_dot_ck
        a[(3, 2)] = 1.0; // m_dot_kr
        a[(3, 10)] = chx.vol * chx.dd_dP_T; // dP_dt

        // Energy balance on cold heat exchanger
        a[(4, 1)] = -h_ck_norm; // m_dot_ck
        a[(4, 2)] = h_kr_norm; // m_dot_kr
        a[(4, 5)] = 1.0 / enth_norm; // Q_dot_k
        a[(4, 10)] = chx.vol * (chx.dens * chx.du_dP_T + chx.inte * chx.dd_dP_T) / enth_norm; // dP_dt

        // Mass balance on regenerator
        a[(5, 2)] = -1.0; // m_dot_kr
        a[(5, 3)] = 1.0; // m_dot_rl
        a[(5, 10)] = regen.vol * regen.dd_dP_T; // dP_dt

        // Energy balance on regenerator
        a[(6, 2)] = -h_kr_norm; // m_dot_kr
        a[(6, 3)] = h_rl_norm; // m_dot_rl
        a[(6, 6)] = 1.0 / enth_norm; // Q_dot_r
        a[(6, 10)] =
            regen.vol * (regen.dens * regen.du_dP_T + regen.inte * regen.dd_dP_T) / enth_norm; // dP_dt

        // Mass balance on hot heat exchanger
        a[(7, 3)] = -1.0; // m_dot_rl
        a[(7, 4)] = 1.0; // m_dot_le
        a[(7, 10)] = hhx.vol * hhx.dd_dP_T; // dP_dt

        // Energy balance on hot heat exchanger
        a[(8, 3)] = -h_rl_norm; // m_dot_rl
        a[(8, 4)] = h_le_norm; // m_dot_le
        a[(8, 7)] = -1.0 / enth_norm; // Q_dot_l
        a[(8, 10)] = hhx.vol * (hhx.dens * hhx.du_dP_T + hhx.inte * hhx.dd_dP_T) / enth_norm; // dP_dt

        // Mass balance on expansion space
        a[(9, 4)] = -1.0; // m_dot_le
        a[(9, 9)] = exp.vol * exp.dd_dT_P; // dTe_dt
        a[(9, 10)] = exp.vol * exp.dd_dP_T; // dP_dt
        b[(9)] = -exp.dens * exp.dV_dt;

        // Energy balance on expansion space
        a[(10, 4)] = -h_le_norm; // m_dot_le
        a[(10, 9)] = exp.vol * (exp.dens * exp.du_dT_P + exp.inte * exp.dd_dT_P) / enth_norm; // dTe_dt
        a[(10, 10)] = exp.vol * (exp.dens * exp.du_dP_T + exp.inte * exp.dd_dP_T) / enth_norm; // dP_dt
        b[(10)] = (-(pres + exp.dens * exp.inte) * exp.dV_dt - exp.Q_dot) / enth_norm;

        Self {
            a,
            b,
            h_comp_norm,
            h_chx_norm,
            h_regen_cold_norm,
            h_regen_hot_norm,
            h_hhx_norm,
            h_exp_norm,
        }
    }

    fn solve(&self) -> Solution {
        let decomp = self.a.lu();
        let x = decomp
            .solve(&self.b)
            .expect("Unable to solve state equations");
        Solution {
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
        }
    }

    fn adjust_flows(&mut self, flow_dir: FlowDirections) {
        // Set the enthalpies at the volume interfaces
        let h_ck_norm = flow_dir.ck.select(self.h_comp_norm, self.h_chx_norm);
        let h_kr_norm = flow_dir.kr.select(self.h_chx_norm, self.h_regen_cold_norm);
        let h_rl_norm = flow_dir.rl.select(self.h_regen_hot_norm, self.h_hhx_norm);
        let h_le_norm = flow_dir.le.select(self.h_hhx_norm, self.h_exp_norm);

        // Adjust the enthalpy entries in the matrix
        self.a[(2, 1)] = h_ck_norm;
        self.a[(4, 1)] = -h_ck_norm;
        self.a[(4, 2)] = h_kr_norm;
        self.a[(6, 2)] = -h_kr_norm;
        self.a[(6, 3)] = h_rl_norm;
        self.a[(8, 3)] = -h_rl_norm;
        self.a[(8, 4)] = h_le_norm;
        self.a[(10, 4)] = -h_le_norm;
    }
}

enum Direction {
    Positive,
    Negative,
    Unknown,
}

impl Direction {
    fn from_value(value: f64) -> Self {
        if (value >= 0.0) {
            Self::Positive
        } else {
            Self::Negative
        }
    }

    fn select(&self, left: f64, right: f64) -> f64 {
        match self {
            Self::Positive => left,
            Self::Negative => right,
            Self::Unknown => 0.5 * (left + right),
        }
    }
}

struct FlowDirections {
    ck: Direction,
    kr: Direction,
    rl: Direction,
    le: Direction,
}

impl FlowDirections {
    fn from_solution(solution: &Solution) -> Self {
        Self {
            ck: Direction::from_value(solution.m_dot_ck),
            kr: Direction::from_value(solution.m_dot_kr),
            rl: Direction::from_value(solution.m_dot_rl),
            le: Direction::from_value(solution.m_dot_le),
        }
    }
}

struct EquationsInputs {
    pres: f64,
    enth_norm: f64,
    flow_dir: FlowDirections,
    comp: WorkingSpaceInputs,
    chx: HeatExchangerInputs,
    regen: RegeneratorInputs,
    hhx: HeatExchangerInputs,
    exp: WorkingSpaceInputs,
}

#[allow(non_snake_case)]
struct WorkingSpaceInputs {
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
struct HeatExchangerInputs {
    vol: f64,
    dens: f64,
    inte: f64,
    enth: f64,
    dd_dP_T: f64,
    du_dP_T: f64,
}

#[allow(non_snake_case)]
struct RegeneratorInputs {
    vol: f64,
    dens: f64,
    inte: f64,
    enth_cold: f64,
    enth_hot: f64,
    dd_dP_T: f64,
    du_dP_T: f64,
}

#[allow(non_snake_case)]
struct Solution {
    m_dot_ck: f64,
    m_dot_kr: f64,
    m_dot_rl: f64,
    m_dot_le: f64,
    Q_dot_k: f64,
    Q_dot_r: f64,
    Q_dot_l: f64,
    dTc_dt: f64,
    dTe_dt: f64,
    dP_dt: f64,
}
