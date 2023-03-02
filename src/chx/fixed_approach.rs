use serde::Deserialize;

use crate::types::ParasiticPower;

use super::{ColdHeatExchanger, State};

#[allow(non_snake_case)]
/// A fixed approach cold heat exchanger.
pub struct FixedApproach {
    R_hyd: f64,
    volume: f64,
    approach: f64,
    parasitics: ParasiticPower,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
/// Configuration for a fixed approach cold heat exchanger.
pub struct Config {
    pub vol: f64,
    pub DT: f64,
    pub R_hyd: f64,
    pub W_parasitic: f64,
}

#[allow(non_snake_case)]
impl FixedApproach {
    /// Create a fixed approach cold heat exchanger.
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

impl ColdHeatExchanger for FixedApproach {
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
}

impl Default for FixedApproach {
    fn default() -> Self {
        Self {
            R_hyd: 0.,
            volume: 4.0e-5,
            approach: 40.,
            parasitics: ParasiticPower::default(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vol: 4e-5,
            DT: 40.,
            R_hyd: 0.,
            W_parasitic: 0.,
        }
    }
}

impl From<Config> for FixedApproach {
    fn from(config: Config) -> Self {
        let parasitics = ParasiticPower {
            mechanical: config.W_parasitic,
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
