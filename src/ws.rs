// mod gpu3;
// mod mod2;
// mod rhombic_drive;
pub mod sinusoidal_drive;

use crate::types::{Environment, ParasiticPower};

pub trait WorkingSpaces {
    /// Returns the frequency (Hz) of the engine
    fn frequency(&self, state: &State) -> f64;

    /// Returns a function that provides `Volumes` as function of time
    fn volumes(&self, state: &State) -> Box<dyn Fn(f64) -> Volumes>;

    /// Returns the thermal resistances of the compression and expansion spaces
    fn thermal_resistance(&self, state: &State) -> ThermalResistance;

    /// Returns the parasitic power associated with the working spaces
    fn parasitics(&self, state: &State) -> Parasitics;

    /// Returns information about the working spaces model
    fn report(&self, state: &State) -> String;
}

/// Volumes (m^3) and their derivatives (m^3/s) of the two spaces
#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Volumes {
    pub V_c: f64,
    pub V_e: f64,
    pub dVc_dt: f64,
    pub dVe_dt: f64,
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

/// Information available to a component for calculating cycle parameters
#[derive(Debug, Clone, Copy)]
pub struct State {
    pub env: Environment,
    pub comp: Average,
    pub exp: Average,
}

impl State {
    /// Create a `ws::State` from sink and source temperatures
    pub fn new(sink: f64, source: f64) -> Self {
        Self {
            env: Environment {
                sink_temp: sink,
                source_temp: source,
            },
            comp: Average {},
            exp: Average {},
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Average {
    // as needed...
}
