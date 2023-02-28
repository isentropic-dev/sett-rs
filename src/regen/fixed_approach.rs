use serde::Deserialize;

use crate::types::ParasiticPower;

use super::{Regenerator, State};

#[allow(non_snake_case)]
/// A fixed approach regenerator.
pub struct FixedApproach {
    R_hyd: f64,
    volume: f64,
    approach: f64,
    parasitics: ParasiticPower,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct FixedApproachConfig {
    pub(crate) vol: f64,
    pub(crate) DT: f64,
    pub(crate) R_hyd: f64,
    pub(crate) Q_parasitic: f64,
}

#[allow(non_snake_case)]
impl FixedApproach {
    /// Create a fixed approach regenerator.
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

impl Regenerator for FixedApproach {
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
            volume: 1.0e-4_f64,
            approach: 10.,
            parasitics: ParasiticPower::default(),
        }
    }
}

impl Default for FixedApproachConfig {
    fn default() -> Self {
        Self {
            vol: 1e-4_f64,
            DT: 10.,
            R_hyd: 0.,
            Q_parasitic: 0.,
        }
    }
}

impl From<FixedApproachConfig> for FixedApproach {
    fn from(config: FixedApproachConfig) -> Self {
        let parasitics = ParasiticPower {
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
