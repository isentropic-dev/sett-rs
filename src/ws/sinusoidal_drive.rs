use std::fmt;

use crate::ParasiticPower;

use super::{Spaces, ThermalResistance, WorkingSpaces};

pub struct SinusoidalDrive {}

impl WorkingSpaces for SinusoidalDrive {
    fn frequency(&self) -> f64 {
        todo!()
    }

    fn spaces(&self, _t: f64) -> Spaces {
        todo!()
    }

    fn thermal_resistance(&self) -> ThermalResistance {
        todo!()
    }

    fn parasitics(&self) -> ParasiticPower {
        todo!()
    }
}

impl fmt::Display for SinusoidalDrive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Sinusoidal drive working spaces")
    }
}
