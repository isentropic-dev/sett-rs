use std::f64::{consts::PI, INFINITY};

use serde::Deserialize;

use crate::types::ParasiticPower;

use super::{CompVolume, ExpVolume, Parasitics, State, ThermalResistance, WorkingSpaces};

pub struct SinusoidalDrive {
    pub frequency: f64,
    pub phase_angle: f64,
    pub comp_geometry: Geometry,
    pub exp_geometry: Geometry,
    pub thermal_resistance: ThermalResistance,
    pub parasitics: Parasitics,
}

pub struct Geometry {
    pub clearance_volume: f64,
    pub swept_volume: f64,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct SinusoidalDriveConfig {
    pub(crate) frequency: f64,
    pub(crate) phase_angle: f64,
    pub(crate) V_swept_c: f64,
    pub(crate) V_clearance_c: f64,
    pub(crate) R_c: f64,
    pub(crate) W_parasitic_c: f64,
    pub(crate) V_swept_e: f64,
    pub(crate) V_clearance_e: f64,
    pub(crate) R_e: f64,
    pub(crate) W_parasitic_e: f64,
    pub(crate) Q_parasitic_e: f64,
}

impl WorkingSpaces for SinusoidalDrive {
    fn frequency(&self, _state: &State) -> f64 {
        self.frequency
    }

    fn volumes(&self, _state: &State) -> Box<(dyn Fn(f64) -> (CompVolume, ExpVolume))> {
        let vol_clear_c = self.comp_geometry.clearance_volume;
        let vol_swept_c = self.comp_geometry.swept_volume;

        let vol_clear_e = self.exp_geometry.clearance_volume;
        let vol_swept_e = self.exp_geometry.swept_volume;

        let omega = 2.0 * PI * self.frequency; // angular velocity (rad/s)
        let phase_angle_rad = self.phase_angle * PI / 180.0; // convert degrees to radians

        Box::new(move |time: f64| {
            let theta = omega * time; // rotation (rad)
            let comp = CompVolume {
                value: vol_clear_c + 0.5 * vol_swept_c * (1.0 + theta.cos()),
                deriv: -0.5 * vol_swept_c * theta.sin() * omega,
            };
            let exp = ExpVolume {
                value: vol_clear_e + 0.5 * vol_swept_e * (1.0 + (theta + phase_angle_rad).cos()),
                deriv: -0.5 * vol_swept_e * (theta + phase_angle_rad).sin() * omega,
            };
            (comp, exp)
        })
    }

    fn thermal_resistance(&self, _state: &State) -> ThermalResistance {
        self.thermal_resistance
    }

    fn parasitics(&self, _state: &State) -> Parasitics {
        self.parasitics
    }
}

impl Default for SinusoidalDriveConfig {
    fn default() -> Self {
        Self {
            frequency: 66.6667,
            phase_angle: 90.,
            V_swept_c: 1.128e-4_f64,
            V_clearance_c: 4.68e-5_f64,
            R_c: INFINITY,
            W_parasitic_c: 0.,
            V_swept_e: 1.128e-4_f64,
            V_clearance_e: 1.68e-5_f64,
            R_e: INFINITY,
            W_parasitic_e: 0.,
            Q_parasitic_e: 0.,
        }
    }
}

impl From<SinusoidalDriveConfig> for SinusoidalDrive {
    fn from(config: SinusoidalDriveConfig) -> Self {
        let parasitics = Parasitics {
            comp: ParasiticPower {
                mechanical: config.W_parasitic_c,
                ..ParasiticPower::default()
            },
            exp: ParasiticPower {
                thermal: config.Q_parasitic_e,
                mechanical: config.W_parasitic_e,
                ..ParasiticPower::default()
            },
        };
        Self {
            frequency: config.frequency,
            phase_angle: config.phase_angle,
            comp_geometry: Geometry {
                clearance_volume: config.V_clearance_c,
                swept_volume: config.V_swept_c,
            },
            exp_geometry: Geometry {
                clearance_volume: config.V_clearance_e,
                swept_volume: config.V_swept_e,
            },
            thermal_resistance: ThermalResistance {
                comp: config.R_c,
                exp: config.R_e,
            },
            parasitics,
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::{assert_relative_eq, relative_eq};

    use crate::engine::Pressure;

    use super::*;

    #[test]
    fn works_as_expected() {
        let clear_vol_c = 1e-5;
        let swept_vol_c = 2e-4;
        let clear_vol_e = 3e-5;
        let swept_vol_e = 4e-4;
        let drive = SinusoidalDrive {
            frequency: 10.0,
            phase_angle: 90.0,
            comp_geometry: Geometry {
                clearance_volume: clear_vol_c,
                swept_volume: swept_vol_c,
            },
            exp_geometry: Geometry {
                clearance_volume: clear_vol_e,
                swept_volume: swept_vol_e,
            },
            thermal_resistance: ThermalResistance::default(),
            parasitics: Parasitics::default(),
        };
        let volumes = drive.volumes(&State {
            pres: Pressure::constant(0.0),
        }); // volumes as a function of time

        let (vol_c_0, vol_e_0) = volumes(0.0); // volumes at time zero
        let (_, vol_e_25) = volumes(0.025); // volumes at 25 ms (1/4 through cycle)
        let (vol_c_50, _) = volumes(0.05); // volumes at 50 ms (1/2 through cycle)
        let (_, vol_e_75) = volumes(0.075); // volumes at 75 ms (3/4 through cycle)
        let (vol_c_final, vol_e_final) = volumes(0.1); // volumes at end of cycle

        // Values at t_initial and t_final should match
        assert_relative_eq!(vol_c_0.value, vol_c_final.value);
        assert_relative_eq!(vol_c_0.deriv, vol_c_final.deriv);
        assert_relative_eq!(vol_e_0.value, vol_e_final.value);
        assert_relative_eq!(vol_e_0.deriv, vol_e_final.deriv);

        // Check values at interesting points in the cycle
        assert_eq!(
            vol_c_0.value,
            clear_vol_c + swept_vol_c,
            "max compression volume is at time zero"
        );
        assert_eq!(
            vol_c_0.deriv, 0.0,
            "compression piston is not moving at time zero"
        );

        assert_eq!(
            vol_e_25.value, clear_vol_e,
            "min expansion volume is 1/4 through cycle" // this is known from 90 deg phase angle
        );
        assert!(
            relative_eq!(vol_e_25.deriv, 0.0),
            "expansion piston is not moving 1/4 through cycle"
        );

        assert_eq!(
            vol_c_50.value, clear_vol_c,
            "min compression volume is 1/2 through cycle"
        );
        assert!(
            relative_eq!(vol_c_50.deriv, 0.0),
            "compression piston is not moving 1/2 through cycle"
        );

        assert_eq!(
            vol_e_75.value,
            clear_vol_e + swept_vol_e,
            "max expansion volume is 3/4 through cycle"
        );
        assert!(
            relative_eq!(vol_e_75.deriv, 0.0),
            "expansion piston is not moving 3/4 through cycle"
        );
    }
}
