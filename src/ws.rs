mod gpu3;
mod mod2;
mod rhombic_drive;
pub mod sinusoidal_drive;

// Export all available working spaces components
pub use gpu3::GPU3;
pub use mod2::Mod2;
pub use rhombic_drive::RhombicDrive;
use serde::Deserialize;
pub use sinusoidal_drive::SinusoidalDrive;

use crate::{engine::Pressure, types::ParasiticPower};

use self::{
    gpu3::GPU3Config, mod2::Mod2Config, rhombic_drive::RhombicDriveConfig,
    sinusoidal_drive::SinusoidalDriveConfig,
};

pub trait WorkingSpaces {
    /// Returns the frequency (Hz) of the engine
    fn frequency(&self, state: &State) -> f64;

    /// Returns a function for `CompressionVolume` and `ExpansionVolume` as a function of time
    fn volumes(&self, state: &State) -> Box<dyn Fn(f64) -> (CompVolume, ExpVolume)>;

    /// Returns the thermal resistances of the compression and expansion spaces
    fn thermal_resistance(&self, state: &State) -> ThermalResistance;

    /// Returns the parasitic power associated with the working spaces
    fn parasitics(&self, state: &State) -> Parasitics;
}

/// Compression space volume (m^3) and its derivative (m^3/s)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CompVolume {
    pub value: f64,
    pub deriv: f64,
}

/// Expansion space volume (m^3) and its derivative (m^3/s)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExpVolume {
    pub value: f64,
    pub deriv: f64,
}

/// Thermal resistance between the gas and the associated heat exchanger temperature
///
/// Thermal resistance have units K/W.  An adiabatic condition is modeled
/// using a thermal resistance value of `f64::INFINITY`, which is the default
/// assumption for both the compression and expansion spaces.
#[derive(Debug, Clone, Copy)]
pub struct ThermalResistance {
    pub comp: f64,
    pub exp: f64,
}

impl Default for ThermalResistance {
    fn default() -> Self {
        Self {
            comp: f64::INFINITY,
            exp: f64::INFINITY,
        }
    }
}

/// Parasitic power (W) related to the two spaces
#[derive(Debug, Clone, Copy, Default)]
pub struct Parasitics {
    pub comp: ParasiticPower,
    pub exp: ParasiticPower,
}

/// Information available to a ws component for calculating its parameters
pub struct State {
    pub pres: Pressure,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WorkingSpacesConfig {
    Sinusoidal(SinusoidalDriveConfig),
    Rhombic(RhombicDriveConfig),
    GPU3(GPU3Config),
    Mod2(Mod2Config),
}
