mod fixed_approach;
mod fixed_conductance;
mod gpu3;
mod mod2;
mod ni_gpu3;
mod ni_mod2;

pub use fixed_approach::FixedApproach;
pub use fixed_conductance::FixedConductance;
pub use gpu3::GPU3;
pub use mod2::Mod2;
pub use ni_gpu3::NuclearIsomerGPU3;
pub use ni_mod2::NuclearIsomerMod2;

use crate::ParasiticPower;

pub trait HotHeatExchanger: std::fmt::Display {
    /// Returns the approach (ΔT) between `T_hot` and `T_l`
    ///
    /// A positive value indicates that `T_hot` is warmer than `T_l`.
    fn approach(&self) -> f64;

    /// Returns the time-discretized pressure drop through the heat exchanger
    fn pressure_drop(&self) -> &[f64];

    /// Returns the parasitic power associated with the heat exchanger
    fn parasitics(&self) -> ParasiticPower;
}
