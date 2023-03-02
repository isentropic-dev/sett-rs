mod fixed_approach;
mod fixed_conductance;
mod gpu3;
mod mod2;
mod ni_gpu3;
mod ni_mod2;

// Export all available hot heat exchanger components
pub use fixed_approach::FixedApproach;
pub use fixed_conductance::FixedConductance;
pub use gpu3::GPU3;
pub use mod2::Mod2;
pub use ni_gpu3::NuclearIsomerGPU3;
pub use ni_mod2::NuclearIsomerMod2;
use serde::Deserialize;

use crate::types::{HeatExchanger, ParasiticPower};

/// Allows a type to act as a hot heat exchanger
pub trait HotHeatExchanger {
    /// Returns the internal volume of the heat exchanger
    ///
    /// The internal volume is the volume in cubic meters (m^3) that is
    /// occupied by the working fluid inside the heat exchanger.
    fn volume(&self) -> f64;

    /// Returns the approach temperature of the heat exchanger
    ///
    /// The approach temperature is the difference in Kelvin (K) between the
    /// hot source (`state.env.source_temp`) and the temperature of the working
    /// fluid in the hot heat exchanger (`T_l`).  A positive value indicates
    /// that `state.env.source_temp` is warmer than `T_l`.
    fn approach(&self, state: &State) -> f64;

    /// Returns the hydraulic resistance in Pa-s/m^3 of the heat exchanger
    fn hydraulic_resistance(&self, state: &State) -> f64;

    /// Returns the parasitic power associated with the heat exchanger
    fn parasitics(&self, state: &State) -> ParasiticPower;
}

/// Information available to a hhx component for calculating its parameters
pub struct State {
    pub hxr: HeatExchanger,
    pub source_temp: f64,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Config {
    FixedApproach(fixed_approach::Config),
    FixedConductance(fixed_conductance::Config),
    GPU3(gpu3::Config),
    Mod2(mod2::Config),
    GPU3NI(ni_gpu3::Config),
    Mod2NI(ni_mod2::Config),
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "model", content = "params")]
pub enum LegacyConfig {
    FixedApproach(fixed_approach::Config),
    FixedConductance(fixed_conductance::Config),
    GPU3(gpu3::Config),
    Mod2(mod2::Config),
    GPU3NI(ni_gpu3::Config),
    Mod2NI(ni_mod2::Config),
}

impl LegacyConfig {
    pub fn into(self) -> Config {
        match self {
            LegacyConfig::FixedApproach(params) => Config::FixedApproach(params),
            LegacyConfig::FixedConductance(params) => Config::FixedConductance(params),
            LegacyConfig::GPU3(params) => Config::GPU3(params),
            LegacyConfig::Mod2(params) => Config::Mod2(params),
            LegacyConfig::GPU3NI(params) => Config::GPU3NI(params),
            LegacyConfig::Mod2NI(params) => Config::Mod2NI(params),
        }
    }
}
