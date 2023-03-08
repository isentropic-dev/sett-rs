use serde::Deserialize;

use crate::types::{Material, ParasiticPower};

use super::{
    types::{FrictionFactorCorrelation, JFactorCorrelation},
    State,
};

pub struct GPU3 {}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    geometry: Geometry,
    Q_parasitic: f64,
    bypass: f64,
    correlationf: FrictionFactorCorrelation,
    correlationj: JFactorCorrelation,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Geometry {
    vol_h: f64,
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
    diameter: f64,
    length: f64,
    number: u32,
}

impl super::Regenerator for GPU3 {
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

    fn initial_approach(&self) -> f64 {
        todo!()
    }
}

impl From<Config> for GPU3 {
    fn from(_: Config) -> Self {
        todo!()
    }
}
