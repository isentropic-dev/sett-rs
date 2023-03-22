use serde::Deserialize;

use crate::types::ParasiticPower;

use super::{HotHeatExchanger, State};

const INITIAL_APPROACH: f64 = 10.;

const DEFAULT_R_HYD: f64 = 0.;
const DEFAULT_VOL: f64 = 1.0e-4;
const DEFAULT_UA: f64 = 100.;
const DEFAULT_W_PARASITIC: f64 = 0.;
const DEFAULT_Q_PARASITIC: f64 = 0.;

#[allow(non_snake_case)]
/// A fixed conductance heat exchanger.
pub struct FixedConductance {
    R_hyd: f64,
    volume: f64,
    UA: f64,
    parasitics: ParasiticPower,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
/// Configuration for a fixed conductance cold heat exchanger.
pub struct Config {
    vol: f64,
    UA: f64,
    R_hyd: f64,
    W_parasitic: f64,
    Q_parasitic: f64,
}

#[allow(non_snake_case)]
impl FixedConductance {
    pub fn new(volume: f64, R_hyd: f64, UA: f64, parasitics: ParasiticPower) -> Self {
        Self {
            R_hyd,
            volume,
            UA,
            parasitics,
        }
    }
}

impl HotHeatExchanger for FixedConductance {
    fn volume(&self) -> f64 {
        self.volume
    }

    #[allow(non_snake_case)]
    fn approach(&self, state: &State) -> f64 {
        let C_dot_avg = state.hxr.cp * state.hxr.m_dot;
        let NTU = self.UA / C_dot_avg;
        let effectiveness = 1. - f64::exp(-NTU);

        (state.hxr.Q_dot / C_dot_avg) * (1. / effectiveness - 1.)
    }

    fn hydraulic_resistance(&self, _state: &State) -> f64 {
        self.R_hyd
    }

    fn parasitics(&self, _state: &State) -> ParasiticPower {
        self.parasitics
    }

    fn initial_approach(&self) -> f64 {
        INITIAL_APPROACH
    }
}

impl Default for FixedConductance {
    fn default() -> Self {
        Self {
            R_hyd: DEFAULT_R_HYD,
            volume: DEFAULT_VOL,
            UA: DEFAULT_UA,
            parasitics: ParasiticPower::default(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vol: DEFAULT_VOL,
            UA: DEFAULT_UA,
            R_hyd: DEFAULT_R_HYD,
            W_parasitic: DEFAULT_W_PARASITIC,
            Q_parasitic: DEFAULT_Q_PARASITIC,
        }
    }
}

impl From<Config> for FixedConductance {
    fn from(config: Config) -> Self {
        let parasitics = ParasiticPower {
            thermal: config.Q_parasitic,
            mechanical: config.W_parasitic,
            ..ParasiticPower::default()
        };
        Self {
            R_hyd: config.R_hyd,
            volume: config.vol,
            UA: config.UA,
            parasitics,
        }
    }
}
