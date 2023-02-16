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
    fn volume(&self, _state: &State) -> f64 {
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

    fn report(&self, _state: &State) -> String {
        todo!()
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
