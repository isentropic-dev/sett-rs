use serde::Deserialize;

use crate::types::ParasiticPower;

use super::{HotHeatExchanger, State};

pub struct GPU3 {}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct GPU3Config {}

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

impl From<GPU3Config> for GPU3 {
    fn from(_: GPU3Config) -> Self {
        todo!()
    }
}
