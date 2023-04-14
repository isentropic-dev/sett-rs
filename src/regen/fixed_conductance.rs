use serde::Deserialize;

use crate::types::ParasiticPower;

use super::{Regenerator, State};

const INITIAL_APPROACH: f64 = 10.;

const DEFAULT_R_HYD: f64 = 0.;
const DEFAULT_VOL: f64 = 1.0e-4;
const DEFAULT_UA: f64 = 70000.;
const DEFAULT_Q_PARASITIC: f64 = 0.;

#[allow(non_snake_case)]
/// A fixed conductance regenerator.
pub struct FixedConductance {
    R_hyd: f64,
    volume: f64,
    UA: f64,
    parasitics: ParasiticPower,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
/// Configuration for a fixed conductance regenerator.
pub struct Config {
    vol: f64,
    UA: f64,
    R_hyd: f64,
    Q_parasitic: f64,
}

#[allow(non_snake_case)]
impl FixedConductance {
    #[must_use]
    pub fn new(volume: f64, R_hyd: f64, UA: f64, parasitics: ParasiticPower) -> Self {
        Self {
            R_hyd,
            volume,
            UA,
            parasitics,
        }
    }
}

impl Regenerator for FixedConductance {
    fn volume(&self) -> f64 {
        self.volume
    }

    #[allow(non_snake_case)]
    fn approach(&self, state: &State) -> f64 {
        let C_dot_avg = state.hxr.cp * state.hxr.m_dot;
        let NTU = self.UA / C_dot_avg;
        let effectiveness = (NTU / 2.) / (1. + (NTU / 2.));

        (1. - effectiveness) * (state.temp_hhx - state.temp_chx)
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
            Q_parasitic: DEFAULT_Q_PARASITIC,
        }
    }
}

impl From<Config> for FixedConductance {
    fn from(config: Config) -> Self {
        let parasitics = ParasiticPower {
            thermal: config.Q_parasitic,
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
