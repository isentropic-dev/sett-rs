mod run;
pub mod state;

use crate::{chx, fluid::Fluid, hhx, regen, state_equations::Values, ws};

/// The components of a Stirling engine
pub struct Components<T: Fluid> {
    pub fluid: T,
    pub ws: Box<dyn ws::WorkingSpaces>,
    pub chx: Box<dyn chx::ColdHeatExchanger>,
    pub regen: Box<dyn regen::Regenerator>,
    pub hhx: Box<dyn hhx::HotHeatExchanger>,
}

/// Represents a Stirling engine running at cyclic steady state
pub struct Engine<T: Fluid> {
    pub components: Components<T>,
    pub state: state::State<T>,
    pub values: Vec<Values>,
}

#[cfg(test)]
mod tests {
    use crate::{
        fluid,
        ws::{sinusoidal_drive::Geometry, Parasitics, ThermalResistance},
    };

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

    fn ws_sinusoidal() -> Box<ws::SinusoidalDrive> {
        Box::new(ws::SinusoidalDrive {
            frequency: 10.0,
            phase_angle: 90.0,
            comp_geometry: Geometry {
                clearance_volume: 1e-5,
                swept_volume: 2e-4,
            },
            exp_geometry: Geometry {
                clearance_volume: 3e-5,
                swept_volume: 4e-4,
            },
            thermal_resistance: ThermalResistance::default(),
            parasitics: Parasitics::default(),
        })
    }

    #[test]
    fn create_components() {
        let fluid = fluid::IdealGas::new("Hydrogen");
        let _components = Components {
            fluid,
            ws: ws_sinusoidal(),
            chx: chx_fixed_approach(),
            regen: regen_fixed_approach(),
            hhx: hhx_fixed_approach(),
        };
    }
}
