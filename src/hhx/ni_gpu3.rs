use std::fmt;

use crate::ParasiticPower;

pub struct NuclearIsomerGPU3 {}

impl super::HotHeatExchanger for NuclearIsomerGPU3 {
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

impl fmt::Display for NuclearIsomerGPU3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "GPU-3 hot heat exchanger with nuclear isomer")
    }
}
