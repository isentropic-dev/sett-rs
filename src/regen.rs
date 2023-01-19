mod fixed_approach;
mod fixed_conductance;
mod gpu3;
mod mod2;

pub use fixed_approach::FixedApproach;
pub use fixed_conductance::FixedConductance;
pub use gpu3::GPU3;
pub use mod2::Mod2;

use crate::ParasiticPower;

pub trait Regenerator {
    /// Returns the approach or minimum ΔT (TODO: decide which it can/should be) of the regenerator
    fn approach(&self) -> f64;

    /// Returns the time-discretized pressure drop through the regenerator
    fn pressure_drop(&self) -> &[f64];

    /// Returns the parasitic power associated with the regenerator
    fn parasitics(&self) -> ParasiticPower;
}
