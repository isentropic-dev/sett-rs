use serde::Deserialize;

use crate::types::{Material, ParasiticPower};

use super::{
    types::{FrictionFactorCorrelation, JFactorCorrelation},
    State,
};

pub struct Mod2 {}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    geometry: Geometry,
    correlationtype: Correlation,
    correlationf: FrictionFactorCorrelation,
    correlationj: JFactorCorrelation,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Geometry {
    mesh: Mesh,
    shell: Shell,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Mesh {
    material: Material,
    D_wire: f64,
    pitch: f64,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Shell {
    material: Material,
    R_sh: f64,
    th_sh_cold: f64,
    th_sh_hot: f64,
    length: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Correlation {
    Steady,
    Oscillating,
}

impl super::Regenerator for Mod2 {
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
