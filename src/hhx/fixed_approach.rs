use serde::Deserialize;

use crate::types::ParasiticPower;

use super::{HotHeatExchanger, State};

const DEFAULT_R_HYD: f64 = 0.;
const DEFAULT_VOL: f64 = 1.0e-4;
const DEFAULT_APPROACH: f64 = 100.;
const DEFAULT_W_PARASITIC: f64 = 0.;
const DEFAULT_Q_PARASITIC: f64 = 0.;

#[allow(non_snake_case)]
/// A fixed approach hot heat exchanger.
pub struct FixedApproach {
    R_hyd: f64,
    volume: f64,
    approach: f64,
    parasitics: ParasiticPower,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub vol: f64,
    pub DT: f64,
    pub R_hyd: f64,
    pub W_parasitic: f64,
    pub Q_parasitic: f64,
}

#[allow(non_snake_case)]
impl FixedApproach {
    /// Create a fixed approach hot heat exchanger.
    ///
    /// # Arguments
    ///
    /// * `volume` - the volume of the heat exchanger (in m^3).
    /// * `R_hyd` - the hydraulic resistance of the heat exchanger (in Pa-s/m^3).
    /// * `approach` - the approach temperature of the heat exchanger (in K).
    /// * `parasitics` - the parasitic losses associated with the heat exchanger (all in W).
    ///
    #[must_use]
    pub fn new(volume: f64, R_hyd: f64, approach: f64, parasitics: ParasiticPower) -> Self {
        Self {
            R_hyd,
            volume,
            approach,
            parasitics,
        }
    }
}

impl HotHeatExchanger for FixedApproach {
    fn volume(&self) -> f64 {
        self.volume
    }

    fn approach(&self, _state: &State) -> f64 {
        self.approach
    }

    fn hydraulic_resistance(&self, _state: &State) -> f64 {
        self.R_hyd
    }

    fn parasitics(&self, _state: &State) -> ParasiticPower {
        self.parasitics
    }

    fn initial_approach(&self) -> f64 {
        self.approach
    }
}

impl Default for FixedApproach {
    fn default() -> Self {
        Self {
            R_hyd: DEFAULT_R_HYD,
            volume: DEFAULT_VOL,
            approach: DEFAULT_APPROACH,
            parasitics: ParasiticPower::default(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vol: DEFAULT_VOL,
            DT: DEFAULT_APPROACH,
            R_hyd: DEFAULT_R_HYD,
            W_parasitic: DEFAULT_W_PARASITIC,
            Q_parasitic: DEFAULT_Q_PARASITIC,
        }
    }
}

impl From<Config> for FixedApproach {
    fn from(config: Config) -> Self {
        let parasitics = ParasiticPower {
            mechanical: config.W_parasitic,
            thermal: config.Q_parasitic,
            ..ParasiticPower::default()
        };
        Self {
            R_hyd: config.R_hyd,
            volume: config.vol,
            approach: config.DT,
            parasitics,
        }
    }
}
