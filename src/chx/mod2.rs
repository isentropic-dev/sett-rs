use serde::Deserialize;

use crate::types::ParasiticPower;

use super::{ColdHeatExchanger, State};

pub struct Mod2 {}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Mod2Config {}

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
}

impl From<Mod2Config> for Mod2 {
    fn from(_: Mod2Config) -> Self {
        todo!()
    }
}
