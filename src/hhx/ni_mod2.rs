use std::fmt;

use crate::ParasiticPower;

pub struct NuclearIsomerMod2 {}

impl super::HotHeatExchanger for NuclearIsomerMod2 {
    fn approach(&self) -> f64 {
        todo!()
    }

    fn pressure_drop(&self) -> &[f64] {
        todo!()
    }

    fn parasitics(&self) -> ParasiticPower {
        todo!()
    }
}

impl fmt::Display for NuclearIsomerMod2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "NASA Mod II hot heat exchanger with nuclear isomer")
    }
}
