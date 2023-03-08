mod fixed_approach;
mod fixed_conductance;
mod gpu3;
mod mod2;

// Export all available cold heat exchanger components
pub use fixed_approach::FixedApproach;
pub use fixed_conductance::FixedConductance;
pub use gpu3::GPU3;
pub use mod2::Mod2;

use serde::Deserialize;

use crate::types::{HeatExchanger, ParasiticPower};

/// Allows a type to act as a cold heat exchanger
pub trait ColdHeatExchanger {
    /// Returns the internal volume of the heat exchanger
    ///
    /// The internal volume is the volume in cubic meters (m^3) that is
    /// occupied by the working fluid inside the heat exchanger.
    fn volume(&self) -> f64;

    /// Returns the approach temperature of the heat exchanger
    ///
    /// The approach temperature is the difference in Kelvin (K) between the
    /// temperature of the working fluid in the cold heat exchanger (`T_k`) and
    /// the cold sink (`state.env.sink_temp`).  A positive value indicates that
    /// `T_k` is warmer than `state.env.sink_temp`.
    fn approach(&self, state: &State) -> f64;

    /// Returns the hydraulic resistance in Pa-s/m^3 of the heat exchanger
    fn hydraulic_resistance(&self, state: &State) -> f64;

    /// Returns the parasitic power associated with the heat exchanger
    fn parasitics(&self, state: &State) -> ParasiticPower;

    /// Returns a reasonable initial approach temperature of the heat
    /// exchanger.
    fn initial_approach(&self, state: &State) -> f64;
}

/// Information available to a chx component for calculating its parameters
pub struct State {
    pub hxr: HeatExchanger,
    pub sink_temp: f64,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Config {
    FixedApproach(fixed_approach::Config),
    FixedConductance(fixed_conductance::Config),
    GPU3(gpu3::Config),
    Mod2(mod2::Config),
}
