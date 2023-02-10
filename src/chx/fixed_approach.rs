use crate::types::ParasiticPower;

use super::{ColdHeatExchanger, State};

pub struct FixedApproach {}

impl ColdHeatExchanger for FixedApproach {
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

    fn report(&self, _state: &State) -> String {
        "Fixed approach cold heat exchanger".to_string()
    }
}
