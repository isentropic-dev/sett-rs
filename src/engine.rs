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
    use crate::{
        types::ParasiticPower,
        ws::{sinusoidal_drive, Parasitics, ThermalResistance},
    };

    use super::*;

    const CHX_VOL: f64 = 4.0e-5_f64;
    const CHX_R_HYD: f64 = 0.;
    const CHX_APPROACH: f64 = 40.;

    const HHX_VOL: f64 = 1.0e-4_f64;
    const HHX_R_HYD: f64 = 0.;
    const HHX_APPROACH: f64 = 100.;

    const REGEN_VOL: f64 = 1.0e-4_f64;
    const REGEN_R_HYD: f64 = 0.;
    const REGEN_APPROACH: f64 = 10.;

    fn chx_fixed_approach() -> Box<chx::FixedApproach> {
        Box::new(chx::FixedApproach::new(
            CHX_VOL,
            CHX_R_HYD,
            CHX_APPROACH,
            ParasiticPower::default(),
        ))
    }

    fn hhx_fixed_approach() -> Box<hhx::FixedApproach> {
        Box::new(hhx::FixedApproach::new(
            HHX_VOL,
            HHX_R_HYD,
            HHX_APPROACH,
            ParasiticPower::default(),
        ))
    }

    fn regen_fixed_approach() -> Box<regen::FixedApproach> {
        Box::new(regen::FixedApproach::new(
            REGEN_VOL,
            REGEN_R_HYD,
            REGEN_APPROACH,
            ParasiticPower::default(),
        ))
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
