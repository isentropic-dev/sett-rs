use serde::Deserialize;

use super::{CompVolume, ExpVolume, Parasitics, State, ThermalResistance, WorkingSpaces};

pub struct GPU3 {}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    frequency: f64,
    V_clearance_c: f64,
    R_c: f64,
    V_clearance_e: f64,
    R_e: f64,
    r_crank: f64,
    L_conn: f64,
    eccentricity: f64,
    D: f64,
    D_dr: f64,
    L: f64,
    h: f64,
}

impl WorkingSpaces for GPU3 {
    fn frequency(&self, _state: &State) -> f64 {
        todo!()
    }

    fn volumes(&self, _state: &State) -> Box<(dyn Fn(f64) -> (CompVolume, ExpVolume))> {
        todo!()
    }

    fn thermal_resistance(&self, _state: &State) -> ThermalResistance {
        todo!()
    }

    fn parasitics(&self, _state: &State) -> Parasitics {
        todo!()
    }
}

impl From<Config> for GPU3 {
    fn from(_: Config) -> Self {
        todo!()
    }
}
