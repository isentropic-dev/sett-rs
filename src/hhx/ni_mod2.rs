use crate::types::ParasiticPower;

use super::{HotHeatExchanger, State};

pub struct NuclearIsomerMod2 {}

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
