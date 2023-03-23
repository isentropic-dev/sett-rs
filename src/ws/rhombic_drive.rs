use std::f64::consts::PI;

use serde::Deserialize;

use crate::types::ParasiticPower;

use super::{CompVolume, ExpVolume, Parasitics, State, ThermalResistance, WorkingSpaces};

const DEFAULT_FREQ: f64 = 50.;
const DEFAULT_V_CLEARANCE_C: f64 = 5.785e-6;
const DEFAULT_R_C: f64 = f64::INFINITY;
const DEFAULT_W_PARASITIC_C: f64 = 0.;
const DEFAULT_V_CLEARANCE_E: f64 = 4.13e-6;
const DEFAULT_R_E: f64 = f64::INFINITY;
const DEFAULT_W_PARASITIC_E: f64 = 0.;
const DEFAULT_Q_PARASITIC_E: f64 = 0.;
const DEFAULT_R_CRANK: f64 = 0.01397;
const DEFAULT_L_CONN: f64 = 0.04602;
const DEFAULT_ECCENTRICITY: f64 = 0.02065;
const DEFAULT_D_P: f64 = 0.0698;
const DEFAULT_D_D: f64 = 0.0696;

#[allow(non_snake_case)]
pub struct RhombicDrive {
    frequency: f64,
    geometry: Geometry,
    parasitics: Parasitics,
    R_comp: f64,
    R_exp: f64,
    V_clearance_c: f64,
    V_clearance_e: f64,
}

#[allow(non_snake_case)]
pub struct Geometry {
    eccentricity: f64,
    r_crank: f64,
    D_p: f64,
    D_d: f64,
    L_conn: f64,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    frequency: f64,
    V_clearance_c: f64,
    R_c: f64,
    W_parasitic_c: f64,
    V_clearance_e: f64,
    R_e: f64,
    W_parasitic_e: f64,
    Q_parasitic_e: f64,
    r_crank: f64,
    L_conn: f64,
    eccentricity: f64,
    D_p: f64,
    D_d: f64,
}

#[allow(non_snake_case)]
impl RhombicDrive {
    #[must_use]
    pub fn new(
        frequency: f64,
        geometry: Geometry,
        parasitics: Parasitics,
        R_comp: f64,
        R_exp: f64,
        V_clearance_c: f64,
        V_clearance_e: f64,
    ) -> Self {
        Self {
            frequency,
            geometry,
            parasitics,
            R_comp,
            R_exp,
            V_clearance_c,
            V_clearance_e,
        }
    }
}

impl WorkingSpaces for RhombicDrive {
    fn frequency(&self, _state: &State) -> f64 {
        self.frequency
    }

    #[allow(non_snake_case)]
    fn volumes(&self, _state: &State) -> Box<(dyn Fn(f64) -> (CompVolume, ExpVolume))> {
        let omega = 2. * PI * self.frequency;

        let eccentricity = self.geometry.eccentricity;
        let r_crank = self.geometry.r_crank;
        let L_conn = self.geometry.L_conn;
        let V_clearance_c = self.V_clearance_c;
        let V_clearance_e = self.V_clearance_e;

        let A_p = PI * self.geometry.D_p.powi(2) / 4.;
        let A_d = PI * self.geometry.D_d.powi(2) / 4.;

        Box::new(move |time: f64| {
            let theta = omega * time;

            let b_theta = (L_conn.powi(2) - (eccentricity + r_crank * theta.cos()).powi(2)).sqrt();
            let b_1 = (L_conn.powi(2) - (eccentricity - r_crank).powi(2)).sqrt();
            let b_2 = ((L_conn - r_crank).powi(2) - eccentricity.powi(2)).sqrt();

            let dVc_dtheta =
                -2. * A_p * r_crank * theta.sin() * (eccentricity + (r_crank * theta.cos()))
                    / b_theta;
            let dVe_dtheta = -((dVc_dtheta * A_d) / (2. * A_p)) - A_d * r_crank * theta.cos();

            (
                CompVolume {
                    value: V_clearance_c + 2. * A_p * (b_1 - b_theta),
                    deriv: dVc_dtheta * omega,
                },
                ExpVolume {
                    value: V_clearance_e + A_d * (b_theta - b_2 - r_crank * theta.sin()),
                    deriv: dVe_dtheta * omega,
                },
            )
        })
    }

    fn thermal_resistance(&self, _state: &State) -> ThermalResistance {
        ThermalResistance {
            comp: self.R_comp,
            exp: self.R_exp,
        }
    }

    fn parasitics(&self, _state: &State) -> Parasitics {
        self.parasitics
    }
}

impl Default for RhombicDrive {
    fn default() -> Self {
        Self {
            geometry: Geometry::default(),
            frequency: DEFAULT_FREQ,
            parasitics: Parasitics {
                comp: ParasiticPower {
                    mechanical: DEFAULT_W_PARASITIC_C,
                    ..ParasiticPower::default()
                },
                exp: ParasiticPower {
                    thermal: DEFAULT_Q_PARASITIC_E,
                    mechanical: DEFAULT_W_PARASITIC_E,
                    ..ParasiticPower::default()
                },
            },
            R_comp: DEFAULT_R_C,
            R_exp: DEFAULT_R_E,
            V_clearance_c: DEFAULT_V_CLEARANCE_C,
            V_clearance_e: DEFAULT_V_CLEARANCE_E,
        }
    }
}

impl Default for Geometry {
    fn default() -> Self {
        Self {
            eccentricity: DEFAULT_ECCENTRICITY,
            r_crank: DEFAULT_R_CRANK,
            D_p: DEFAULT_D_P,
            D_d: DEFAULT_D_D,
            L_conn: DEFAULT_L_CONN,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            frequency: DEFAULT_FREQ,
            V_clearance_c: DEFAULT_V_CLEARANCE_C,
            R_c: DEFAULT_R_C,
            W_parasitic_c: DEFAULT_W_PARASITIC_C,
            V_clearance_e: DEFAULT_V_CLEARANCE_E,
            R_e: DEFAULT_R_E,
            W_parasitic_e: DEFAULT_W_PARASITIC_E,
            Q_parasitic_e: DEFAULT_Q_PARASITIC_E,
            r_crank: DEFAULT_R_CRANK,
            L_conn: DEFAULT_L_CONN,
            eccentricity: DEFAULT_ECCENTRICITY,
            D_p: DEFAULT_D_P,
            D_d: DEFAULT_D_D,
        }
    }
}

impl From<Config> for RhombicDrive {
    fn from(config: Config) -> Self {
        let geometry = Geometry {
            eccentricity: config.eccentricity,
            r_crank: config.r_crank,
            D_p: config.D_p,
            D_d: config.D_p,
            L_conn: config.L_conn,
        };
        let parasitics = Parasitics {
            comp: ParasiticPower {
                mechanical: config.W_parasitic_c,
                ..ParasiticPower::default()
            },
            exp: ParasiticPower {
                thermal: config.Q_parasitic_e,
                mechanical: config.W_parasitic_e,
                ..ParasiticPower::default()
            },
        };
        Self {
            geometry,
            frequency: config.frequency,
            parasitics,
            R_comp: config.R_c,
            R_exp: config.R_e,
            V_clearance_c: config.V_clearance_c,
            V_clearance_e: config.V_clearance_e,
        }
    }
}
