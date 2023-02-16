use crate::types::ParasiticPower;

use super::{HotHeatExchanger, State};

pub struct NuclearIsomerGPU3 {}

impl HotHeatExchanger for NuclearIsomerGPU3 {
    fn volume(&self, _state: &State) -> f64 {
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
