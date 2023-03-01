use serde::Deserialize;

use crate::types::ParasiticPower;

use super::State;

pub struct Mod2 {}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Config {}

impl super::Regenerator for Mod2 {
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

impl From<Config> for Mod2 {
    fn from(_: Config) -> Self {
        todo!()
    }
}
