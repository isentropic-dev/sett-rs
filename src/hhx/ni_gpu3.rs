use serde::Deserialize;

use crate::types::ParasiticPower;

use super::{HotHeatExchanger, State};

pub struct NuclearIsomerGPU3 {}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    R_f: f64,
    L_f: f64,
    R_regen: f64,
    D_outer: f64,
    D_inner: f64,
    k_f: f64,
    roughness: f64,
    N_total: u32,
    vol_h: f64,
    R_ins: f64,
    W_parasitic: f64,
}

impl HotHeatExchanger for NuclearIsomerGPU3 {
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

impl From<Config> for NuclearIsomerGPU3 {
    fn from(_: Config) -> Self {
        todo!()
    }
}
