use crate::types::ParasiticPower;

use super::{Regenerator, State};

pub struct FixedApproach {}

impl Regenerator for FixedApproach {
    fn volume(&self, _state: &State) -> f64 {
        todo!()
    }

    fn approach(&self, _state: &State) -> f64 {
        todo!()
    }

    fn hydraulic_resistance(&self, _state: &State) -> f64 {
        todo!()
    }

    fn pressure_drop(&self, _state: &State) -> &[f64] {
        todo!()
    }

    fn parasitics(&self, _state: &State) -> ParasiticPower {
        todo!()
    }

    fn report(&self, _state: &State) -> String {
        "Fixed approach regenerator".to_string()
    }
}
