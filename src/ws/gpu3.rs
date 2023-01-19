use std::fmt;

use crate::ParasiticPower;

use super::{Spaces, ThermalResistance, WorkingSpaces};

pub struct GPU3 {}

impl WorkingSpaces for GPU3 {
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

impl fmt::Display for GPU3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "GPU-3 working spaces")
    }
}
