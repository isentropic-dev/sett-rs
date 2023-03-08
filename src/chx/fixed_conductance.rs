use serde::Deserialize;

use crate::types::ParasiticPower;

use super::{ColdHeatExchanger, State};

pub struct FixedConductance {}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
/// Configuration for a fixed conductance cold heat exchanger.
pub struct Config {
    vol: f64,
    UA: f64,
    R_hyd: f64,
    W_parasitic: f64,
}

impl ColdHeatExchanger for FixedConductance {
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

impl From<Config> for FixedConductance {
    fn from(_: Config) -> Self {
        todo!()
    }
}
