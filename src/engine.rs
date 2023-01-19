use std::fmt;

use crate::{chx, fluid, hhx, regen, ws};

/// Represents a Stirling engine
pub struct Engine<T: fluid::WorkingFluid> {
    pub fluid: T,
    pub ws: Box<dyn ws::WorkingSpaces>,
    pub chx: Box<dyn chx::ColdHeatExchanger>,
    pub regen: Box<dyn regen::Regenerator>,
    pub hhx: Box<dyn hhx::HotHeatExchanger>,
}

impl<T: fluid::WorkingFluid> fmt::Display for Engine<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "An engine!")?;
        writeln!(f, "Fluid: {}", self.fluid.describe())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{chx, fluid};

    #[test]
    fn create_engine() {
        let fluid = fluid::IdealGas::new("Hydrogen");
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
