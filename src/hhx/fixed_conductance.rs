use super::{HotHeatExchanger, State};

pub struct FixedConductance {}

impl HotHeatExchanger for FixedConductance {
    fn volume(&self, _state: &State) -> f64 {
        todo!()
    }

    fn approach(&self, _state: &State) -> f64 {
        todo!()
    }

    fn hydraulic_resistance(&self, _state: &State) -> f64 {
        todo!()
    }

    fn parasitics(&self, _state: &State) -> crate::types::ParasiticPower {
        todo!()
    }
}
