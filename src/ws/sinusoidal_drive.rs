use std::f64::consts::PI;

use super::{Parasitics, ThermalResistance, Volumes, WorkingSpaces};

#[allow(non_snake_case)]
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

impl WorkingSpaces for SinusoidalDrive {
    fn frequency(&self, _state: &super::State) -> f64 {
        self.frequency
    }

    fn volumes(&self, _state: &super::State) -> Box<(dyn Fn(f64) -> Volumes)> {
        let vol_clear_c = self.comp_geometry.clearance_volume;
        let vol_swept_c = self.comp_geometry.swept_volume;
        let vol_clear_e = self.exp_geometry.clearance_volume;
        let vol_swept_e = self.exp_geometry.swept_volume;
        let omega = 2.0 * PI * self.frequency; // angular velocity (rad/s)
        let phase_angle_rad = self.phase_angle * PI / 180.0; // convert degrees to radians
        Box::new(move |time: f64| {
            let theta = omega * time; // rotation (rad)
            Volumes {
                V_c: vol_clear_c + 0.5 * vol_swept_c * (1.0 + theta.cos()),
                V_e: vol_clear_e + 0.5 * vol_swept_e * (1.0 + (theta + phase_angle_rad).cos()),
                dVc_dt: -0.5 * vol_swept_c * theta.sin() * omega,
                dVe_dt: -0.5 * vol_swept_e * (theta + phase_angle_rad).sin() * omega,
            }
        })
    }

    fn thermal_resistance(&self, _state: &super::State) -> ThermalResistance {
        self.thermal_resistance
    }

    fn parasitics(&self, _state: &super::State) -> Parasitics {
        self.parasitics
    }

    fn report(&self, _state: &super::State) -> String {
        "Sinusoidal drive working spaces".to_string()
    }
}

#[cfg(test)]
mod tests {
    use approx::{assert_relative_eq, relative_eq};

    use crate::ws::State;

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
        let state = State::new(0.0, 0.0); // required but not used by this component
        let volumes = drive.volumes(&state); // volumes as a function of time

        let vol_0 = volumes(0.0); // volumes at time zero
        let vol_25 = volumes(0.025); // volumes at 25 ms (1/4 through cycle)
        let vol_50 = volumes(0.05); // volumes at 50 ms (1/2 through cycle)
        let vol_75 = volumes(0.075); // volumes at 75 ms (3/4 through cycle)
        let vol_final = volumes(0.1); // volumes at end of cycle

        // Values at t_initial and t_final should match
        assert_relative_eq!(vol_0.V_c, vol_final.V_c);
        assert_relative_eq!(vol_0.V_e, vol_final.V_e);
        assert_relative_eq!(vol_0.dVc_dt, vol_final.dVc_dt);
        assert_relative_eq!(vol_0.dVe_dt, vol_final.dVe_dt);

        // Check values at interesting points in the cycle
        assert_eq!(
            vol_0.V_c,
            clear_vol_c + swept_vol_c,
            "max compression volume is at time zero"
        );
        assert_eq!(
            vol_0.dVc_dt, 0.0,
            "compression piston is not moving at time zero"
        );

        assert_eq!(
            vol_25.V_e, clear_vol_e,
            "min expansion volume is 1/4 through cycle" // this is known from 90 deg phase angle
        );
        assert!(
            relative_eq!(vol_25.dVe_dt, 0.0),
            "expansion piston is not moving 1/4 through cycle"
        );

        assert_eq!(
            vol_50.V_c, clear_vol_c,
            "min compression volume is 1/2 through cycle"
        );
        assert!(
            relative_eq!(vol_50.dVc_dt, 0.0),
            "compression piston is not moving 1/2 through cycle"
        );

        assert_eq!(
            vol_75.V_e,
            clear_vol_e + swept_vol_e,
            "max expansion volume is 3/4 through cycle"
        );
        assert!(
            relative_eq!(vol_75.dVe_dt, 0.0),
            "expansion piston is not moving 3/4 through cycle"
        );
    }
}
