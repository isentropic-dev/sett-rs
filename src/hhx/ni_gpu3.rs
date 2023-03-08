use serde::Deserialize;

use crate::types::ParasiticPower;

use super::{HotHeatExchanger, State};

pub struct NuclearIsomerGPU3 {}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Config {}

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

    fn initial_approach(&self) -> f64 {
        todo!()
    }
}

impl From<Config> for NuclearIsomerGPU3 {
    fn from(_: Config) -> Self {
        todo!()
    }
}
