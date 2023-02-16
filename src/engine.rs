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
    use crate::ws::{sinusoidal_drive, Parasitics, ThermalResistance};

    use super::*;

    fn chx_fixed_approach() -> Box<chx::FixedApproach> {
        Box::new(chx::FixedApproach::default())
    }

    fn hhx_fixed_approach() -> Box<hhx::FixedApproach> {
        Box::new(hhx::FixedApproach::default())
    }

    fn regen_fixed_approach() -> Box<regen::FixedApproach> {
        Box::new(regen::FixedApproach::default())
    }

    #[test]
    fn create_engine() {
        let fluid = Box::new(fluid::IdealGas::new("Hydrogen"));
        let ws = Box::new(sinusoidal_drive::SinusoidalDrive {
            frequency: 10.0,
            phase_angle: 90.0,
            comp_geometry: sinusoidal_drive::Geometry {
                clearance_volume: 1e-5,
                swept_volume: 2e-4,
            },
            exp_geometry: sinusoidal_drive::Geometry {
                clearance_volume: 3e-5,
                swept_volume: 4e-4,
            },
            thermal_resistance: ThermalResistance::default(),
            parasitics: Parasitics::default(),
        });
        let _engine = Engine {
            fluid,
            ws,
            chx: chx_fixed_approach(),
            regen: regen_fixed_approach(),
            hhx: hhx_fixed_approach(),
        };
    }
}
