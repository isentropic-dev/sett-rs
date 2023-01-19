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
    /// Returns the approach (ΔT) between `T_k` and `T_cold`
    ///
    /// A positive value indicates that `T_k` is warmer than `T_cold`.
    fn approach(&self) -> f64;

    /// Returns the time-discretized pressure drop through the heat exchanger
    fn pressure_drop(&self) -> &[f64];

    /// Returns the parasitic power associated with the heat exchanger
    fn parasitics(&self) -> ParasiticPower;
}
