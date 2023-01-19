mod fixed_approach;
mod fixed_conductance;
mod gpu3;
mod mod2;

pub use fixed_approach::FixedApproach;
pub use fixed_conductance::FixedConductance;
pub use gpu3::GPU3;
pub use mod2::Mod2;

use crate::ParasiticPower;

pub trait Regenerator: std::fmt::Display {
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
    fn approach(&self) -> f64;

    /// Returns the time-discretized pressure drop through the regenerator
    fn pressure_drop(&self) -> &[f64];

    /// Returns the parasitic power associated with the regenerator
    fn parasitics(&self) -> ParasiticPower;
}
