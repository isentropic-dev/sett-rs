use std::fmt;

use crate::ParasiticPower;

pub struct GPU3 {}

impl super::Regenerator for GPU3 {
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

impl fmt::Display for GPU3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "GPU-3 regenerator")
    }
}
