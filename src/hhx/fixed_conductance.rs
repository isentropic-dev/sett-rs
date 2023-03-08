use serde::Deserialize;

use super::{HotHeatExchanger, State};

pub struct FixedConductance {}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    vol: f64,
    UA: f64,
    R_hyd: f64,
    W_parasitic: f64,
    Q_parasitic: f64,
}

impl HotHeatExchanger for FixedConductance {
    fn volume(&self) -> f64 {
        todo!()
    }

    fn approach(&self, _state: &State) -> f64 {
        todo!()
    }

    fn hydraulic_resistance(&self, _state: &State) -> f64 {
        todo!()
    }

    fn parasitics(&self, _state: &State) -> crate::types::ParasiticPower {
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
