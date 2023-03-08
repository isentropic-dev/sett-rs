use serde::Deserialize;

use crate::types::{Material, ParasiticPower};

use super::{HotHeatExchanger, State};

pub struct Mod2 {}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
/// Configuration for a Mod II/I hot heat exchanger.
pub struct Config {
    geometry: Geometry,
    correlation: Correlation,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Geometry {
    tubes: Tubes,
    shell: Shell,
    fins: Fins,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Tubes {
    L_front: f64,
    L_rear: f64,
    L_inactive: f64,
    D_outer: f64,
    D_inner: f64,
    roughness: f64,
    N_total: u32,
    materialtube: Material,
    materialfin: Material,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Shell {
    R_outer: f64,
    R_inner: f64,
    V_header: f64,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Fins {
    thickness: f64,
    pitch: f64,
    L_fin: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Correlation {
    Oscillating,
    Steady,
}

impl HotHeatExchanger for Mod2 {
    fn volume(&self) -> f64 {
        todo!()
    }
    fn approach(&self, _state: &State) -> f64 {
        todo!()
    }

    fn hydraulic_resistance(&self, _state: &State) -> f64 {
        todo!()
    }

    fn parasitics(&self, _state: &State) -> ParasiticPower {
        todo!()
    }
}

impl From<Config> for Mod2 {
    fn from(_: Config) -> Self {
        todo!()
    }
}
