use serde::Deserialize;

use crate::types::Material;

use super::{CompVolume, ExpVolume, Parasitics, State, ThermalResistance, WorkingSpaces};

pub struct Mod2 {}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    frequency: f64,
    phaseAngle: f64,
    D: f64,
    h: f64,
    L: f64,
    stroke: f64,
    V_clearance_c: f64,
    R_c: f64,
    V_clearance_e: f64,
    R_e: f64,
    material_p: Material,
    material_c: Material,
    th_pw: f64,
    th_cw: f64,
    L_cond: f64,
    e: f64,
}

impl WorkingSpaces for Mod2 {
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

impl From<Config> for Mod2 {
    fn from(_: Config) -> Self {
        todo!()
    }
}
