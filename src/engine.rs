use crate::{chx, fluid, hhx, regen, ws};

/// Represents a Stirling engine
pub struct Engine {
    pub fluid: Box<dyn fluid::WorkingFluid>,
    pub ws: Box<dyn ws::WorkingSpaces>,
    pub chx: Box<dyn chx::ColdHeatExchanger>,
    pub regen: Box<dyn regen::Regenerator>,
    pub hhx: Box<dyn hhx::HotHeatExchanger>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_engine() {
        let fluid = Box::new(fluid::IdealGas::new("Hydrogen"));
        let ws = Box::new(
            ws::SinusoidalDrive::builder()
                .with_frequency(75.0) // 4,500 rpm
                .with_phase_angle(90.0)
                .with_compression_volumes(1.0, 1.0)
                .with_expansion_volumes(1.0, 1.0)
                .build(),
        );
        let chx = Box::new(chx::FixedApproach {});
        let regen = Box::new(regen::FixedApproach {});
        let hhx = Box::new(hhx::FixedApproach {});
        let _engine = Engine {
            fluid,
            ws,
            chx,
            regen,
            hhx,
        };
    }
}
