use serde::Deserialize;

use crate::types::ParasiticPower;

use super::{ColdHeatExchanger, State};

pub struct GPU3 {}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
/// Configuration for a GPU3 cold heat exchanger.
pub struct Config {
    length_total: f64,
    length_ht: f64,
    D_inner: f64,
    D_outer: f64,
    N_total: u32,
    N_shell: u32,
    D_sh: f64,
    Ac_d: f64,
    roughness: f64,
    vol_h: f64,
    m_dot_w: f64,
    coolant: Coolant,
    m_dot_a: f64,
    UA_a: f64,
    W_parasitic: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Coolant {
    Water,
}

impl ColdHeatExchanger for GPU3 {
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

impl From<Config> for GPU3 {
    fn from(_: Config) -> Self {
        todo!()
    }
}
