use serde::Deserialize;

use crate::types::ParasiticPower;

use super::{ColdHeatExchanger, State};

pub struct GPU3 {}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Config {}

impl ColdHeatExchanger for GPU3 {
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

impl From<Config> for GPU3 {
    fn from(_: Config) -> Self {
        todo!()
    }
}
