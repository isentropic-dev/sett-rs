mod fixed_approach;
// mod fixed_conductance;
// mod gpu3;
// mod mod2;

// Export all available regenerator components
pub use fixed_approach::FixedApproach;
// pub use fixed_conductance::FixedConductance;
// pub use gpu3::GPU3;
// pub use mod2::Mod2;

use crate::types::{Environment, ParasiticPower};

/// Allows a type to act as a regenerator
pub trait Regenerator {
    /// Returns the void volume of the heat exchanger
    ///
    /// The void volume is the volume in cubic meters (m^3) that is
    /// occupied by the working fluid inside the regenerator.
    fn volume(&self, state: &State) -> f64;

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
