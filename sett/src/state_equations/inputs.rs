use serde::Deserialize;

/// Inputs required to generate the `Ax=b` system of state equations
#[derive(Debug, Clone, Deserialize)]
pub struct Inputs {
    pub pres: f64,
    pub enth_norm: f64,
    pub comp: WorkingSpace,
    pub chx: HeatExchanger,
    pub regen: Regenerator,
    pub hhx: HeatExchanger,
    pub exp: WorkingSpace,
}

/// State equation inputs related to the working spaces
#[allow(non_snake_case)]
#[derive(Debug, Clone, Deserialize)]
pub struct WorkingSpace {
    pub vol: f64,
    pub dens: f64,
    pub inte: f64,
    pub enth: f64,
    pub dd_dP_T: f64,
    pub dd_dT_P: f64,
    pub du_dP_T: f64,
    pub du_dT_P: f64,
    pub dV_dt: f64,
    pub Q_dot: f64,
}

/// State equation inputs related to the heat exchangers
#[allow(non_snake_case)]
#[derive(Debug, Clone, Deserialize)]
pub struct HeatExchanger {
    pub vol: f64,
    pub dens: f64,
    pub inte: f64,
    pub enth: f64,
    pub dd_dP_T: f64,
    pub du_dP_T: f64,
}

/// State equation inputs related to the regenerator
#[allow(non_snake_case)]
#[derive(Debug, Clone, Deserialize)]
pub struct Regenerator {
    pub vol: f64,
    pub dens: f64,
    pub inte: f64,
    pub enth_cold: f64,
    pub enth_hot: f64,
    pub dd_dP_T: f64,
    pub du_dP_T: f64,
}
