mod fixed_approach;
mod fixed_conductance;
mod gpu3;
mod mod2;

use crate::types::{Environment, ParasiticPower};

// Export all available cold heat exchanger components
pub use fixed_approach::FixedApproach;
pub use fixed_conductance::FixedConductance;
pub use gpu3::GPU3;
pub use mod2::Mod2;

/// Allows a type to act as a cold heat exchanger
pub trait ColdHeatExchanger {
    /// Returns the internal volume of the heat exchanger
    ///
    /// The internal volume is the volume in cubic meters (m^3) that is
    /// occupied by the working fluid inside the heat exchanger.
    fn volume(&self, state: &State) -> f64;

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
