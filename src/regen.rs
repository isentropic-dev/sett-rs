mod fixed_approach;
mod fixed_conductance;
mod gpu3;
mod mod2;

// Export all available regenerator components
pub use fixed_approach::FixedApproach;
pub use fixed_conductance::FixedConductance;
pub use gpu3::GPU3;
pub use mod2::Mod2;

use serde::Deserialize;

use crate::types::{HeatExchanger, ParasiticPower};

/// Allows a type to act as a regenerator
pub trait Regenerator {
    /// Returns the void volume of the heat exchanger
    ///
    /// The void volume is the volume in cubic meters (m^3) that is
    /// occupied by the working fluid inside the regenerator.
    fn volume(&self) -> f64;

    /// Returns the approach temperature of the regenerator
    ///
    /// The approach temperature is the minimum temperature difference in
    /// Kelvin (K) between either the temperature of the working fluid in
    /// the cold heat exchanger (`T_k`) and the cold side of the regenerator
    /// (`T_r_cold`) or the temperature of the working fluid in the hot heat
    /// exhanger (`T_l`) and the hot side of the regeneratore (`T_r_hot`).
    fn approach(&self, state: &State) -> f64;

    /// Returns the hydraulic resistance in Pa-s/m^3 of the regenerator
    fn hydraulic_resistance(&self, state: &State) -> f64;

    /// Returns the parasitic power associated with the regenerator
    fn parasitics(&self, state: &State) -> ParasiticPower;
}

/// Information available to a regenerator component for calculating its parameters
pub struct State {
    pub hxr: HeatExchanger,
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

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "model", content = "params")]
pub enum LegacyConfig {
    FixedApproach(fixed_approach::Config),
    FixedConductance(fixed_conductance::Config),
    GPU3(gpu3::Config),
    Mod2(mod2::Config),
}

impl LegacyConfig {
    pub fn into(self) -> Config {
        match self {
            LegacyConfig::FixedApproach(params) => Config::FixedApproach(params),
            LegacyConfig::FixedConductance(params) => Config::FixedConductance(params),
            LegacyConfig::GPU3(params) => Config::GPU3(params),
            LegacyConfig::Mod2(params) => Config::Mod2(params),
        }
    }
}
