use serde::Deserialize;

use crate::types::ParasiticPower;

use super::State;

pub struct FixedConductance {}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct FixedConductanceConfig {}

impl super::Regenerator for FixedConductance {
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

impl From<FixedConductanceConfig> for FixedConductance {
    fn from(_: FixedConductanceConfig) -> Self {
        todo!()
    }
}
