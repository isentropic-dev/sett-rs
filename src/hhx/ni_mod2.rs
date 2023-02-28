use serde::Deserialize;

use crate::types::ParasiticPower;

use super::{HotHeatExchanger, State};

pub struct NuclearIsomerMod2 {}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct NuclearIsomerMod2Config {}

impl HotHeatExchanger for NuclearIsomerMod2 {
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

impl From<NuclearIsomerMod2Config> for NuclearIsomerMod2 {
    fn from(_: NuclearIsomerMod2Config) -> Self {
        todo!()
    }
}
