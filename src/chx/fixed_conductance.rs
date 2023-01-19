use std::fmt;

use crate::ParasiticPower;

pub struct FixedConductance {}

impl super::ColdHeatExchanger for FixedConductance {
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

impl fmt::Display for FixedConductance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Fixed conductance cold heat exchanger")
    }
}