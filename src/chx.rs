mod fixed_approach;
mod fixed_conductance;
mod gpu3;
mod mod2;

pub use fixed_approach::FixedApproach;
pub use fixed_conductance::FixedConductance;
pub use gpu3::GPU3;
pub use mod2::Mod2;

use crate::ParasiticPower;

pub trait ColdHeatExchanger: std::fmt::Display {
    /// Returns the internal volume of the heat exchanger
    ///
    /// The internal volume is the volume in cubic meters (m^3) that is
    /// occupied by the working fluid inside the heat exchanger.
    fn volume(&self) -> f64;

    /// Returns the approach temperature of the heat exchanger
    ///
    /// The approach temperature is the difference in Kelvin (K) between the
    /// temperature of the working fluid in the cold heat exchanger (`T_k`)
    /// and the cold sink (`T_cold`).  A positive value indicates that `T_k` is
    /// warmer than `T_cold`.
    fn approach(&self) -> f64;

    /// Returns the time-discretized pressure drop through the heat exchanger
    fn pressure_drop(&self) -> &[f64];

    /// Returns the parasitic power associated with the heat exchanger
    fn parasitics(&self) -> ParasiticPower;
}
