use serde::Deserialize;

use crate::types::{Material, ParasiticPower};

use super::{ColdHeatExchanger, State};

pub struct Mod2 {}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
/// Configuration for a Mod II/I cold heat exchanger.
pub struct Config {
    geometry: Geometry,
    m_dot_p_fs: f64,
    W_dot_p_fs: f64,
    n_fs: f64,
    fluid: Fluid,
    correlation: Correlation,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Geometry {
    tubes: Tubes,
    shell: Shell,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Tubes {
    length: f64,
    length_ht: f64,
    D_outer: f64,
    D_inner: f64,
    N_total: u32,
    roughness: f64,
    material: Material,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Shell {
    R_inner: f64,
    V_header: f64,
    Ac_header: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Fluid {
    Water,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Correlation {
    Oscillating,
    Steady,
}

impl ColdHeatExchanger for Mod2 {
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

impl From<Config> for Mod2 {
    fn from(_: Config) -> Self {
        todo!()
    }
}
