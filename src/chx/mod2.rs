use std::fmt;

use crate::ParasiticPower;

pub struct Mod2 {}

impl super::ColdHeatExchanger for Mod2 {
    fn volume(&self) -> f64 {
        todo!()
    }

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

impl fmt::Display for Mod2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "NASA Mod II cold heat exchanger")
    }
}
