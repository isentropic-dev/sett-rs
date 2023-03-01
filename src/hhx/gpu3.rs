use serde::Deserialize;

use crate::types::ParasiticPower;

use super::{HotHeatExchanger, State};

pub struct GPU3 {}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Config {}

impl HotHeatExchanger for GPU3 {
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
