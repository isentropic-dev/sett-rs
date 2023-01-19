use std::fmt;

use crate::{chx, fluid, hhx, regen, ws};

/// Represents a Stirling engine
pub struct Engine {
    pub fluid: Box<dyn fluid::WorkingFluid>,
    pub ws: Box<dyn ws::WorkingSpaces>,
    pub chx: Box<dyn chx::ColdHeatExchanger>,
    pub regen: Box<dyn regen::Regenerator>,
    pub hhx: Box<dyn hhx::HotHeatExchanger>,
}

impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "An engine!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{chx, fluid};

    #[test]
    fn create_engine() {
        let fluid = Box::new(fluid::IdealGas::new("Hydrogen"));
        let ws = Box::new(ws::SinusoidalDrive {});
        let chx = Box::new(chx::FixedApproach {});
        let regen = Box::new(regen::FixedApproach {});
        let hhx = Box::new(hhx::FixedApproach {});
        let engine = Engine {
            fluid,
            ws,
            chx,
            regen,
            hhx,
        };
        println!("{engine}");
    }
}
