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

use crate::types::{Environment, ParasiticPower};

/// Allows a type to act as a hot heat exchanger
pub trait HotHeatExchanger {
    /// Returns the internal volume of the heat exchanger
    ///
    /// The internal volume is the volume in cubic meters (m^3) that is
    /// occupied by the working fluid inside the heat exchanger.
    fn volume(&self, state: &State) -> f64;

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

    /// Returns information about the heat exchanger
    fn report(&self, state: &State) -> String;
}

/// Information available to a component for calculating cycle parameters
pub struct State {
    env: Environment,
    avg: Average,
}

#[allow(non_snake_case)]
pub struct Average {
    temp: f64,
    pres: f64,
    dens: f64,
    inte: f64, // TODO: needed?
    enth: f64, // TODO: needed?
    cp: f64,
    m_dot: f64,
    Q_dot: f64,
}
