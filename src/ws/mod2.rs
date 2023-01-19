use crate::ParasiticPower;

use super::{Spaces, ThermalResistance, WorkingSpaces};

pub struct Mod2 {}

impl WorkingSpaces for Mod2 {
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
