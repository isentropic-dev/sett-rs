use crate::types::ParasiticPower;

use super::{HotHeatExchanger, State};

pub struct FixedApproach {}

impl HotHeatExchanger for FixedApproach {
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
        "Fixed approach hot heat exchanger".to_string()
    }
}
