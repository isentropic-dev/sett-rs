mod gpu3;
mod mod2;
mod rhombic_drive;
mod sinusoidal_drive;

pub use gpu3::GPU3;
pub use mod2::Mod2;
pub use rhombic_drive::RhombicDrive;
pub use sinusoidal_drive::SinusoidalDrive;

use crate::ParasiticPower;

pub trait WorkingSpaces: std::fmt::Display {
    /// Returns the frequency of the engine
    fn frequency(&self) -> f64;

    /// Returns working space volumes and derivatives as a function of time `t`
    fn spaces(&self, t: f64) -> Spaces;

    /// Returns the thermal resistances of the compression and expansion spaces
    fn thermal_resistance(&self) -> ThermalResistance;

    /// Returns the parasitic power associated with the heat exchanger
    fn parasitics(&self) -> ParasiticPower;

    /// Indicates whether the working spaces model is converged
    ///
    /// This trait method is optional and is typically only used for free
    /// piston Stirling engine models.  For kinematic (direct-drive) Stirling
    /// engines, the working spaces model is always converged and this trait
    /// method is not required.
    fn is_converged(&self) -> bool {
        true
    }
}

/// Thermal resistance between the gas and the associated heat exchanger temperature
pub struct ThermalResistance {
    pub compression: f64,
    pub expansion: f64,
}

pub struct Spaces {
    pub compression: Space,
    pub expansion: Space,
}

pub struct Space {
    pub volume: f64,
    pub derivative: f64,
}
