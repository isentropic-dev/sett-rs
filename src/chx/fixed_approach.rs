use std::fmt;

use crate::ParasiticPower;

pub struct FixedApproach {}

impl super::ColdHeatExchanger for FixedApproach {
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

impl fmt::Display for FixedApproach {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Fixed approach cold heat exchanger")
    }
}
