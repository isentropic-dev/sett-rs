use std::f64::consts::PI;

use crate::ws::ParasiticPower;

use super::{Parasitics, ThermalResistance, Volumes, WorkingSpaces};

#[allow(non_snake_case)]
pub struct SinusoidalDrive {
    // TODO: clean up internal state and change these names
    frequency: f64,
    V_swept_c: f64,
    V_swept_e: f64,
    V_clearance_c: f64,
    V_clearance_e: f64,
    R_c: f64,
    R_e: f64,
    phase_angle: f64,
    omega: f64,
    W_parasitic_c: f64,
    W_parasitic_e: f64,
    Q_parasitic_e: f64,
}

impl WorkingSpaces for SinusoidalDrive {
    fn frequency(&self, _state: &super::State) -> f64 {
        self.frequency
    }

    fn volumes(&self, _state: &super::State) -> Box<(dyn Fn(f64) -> Volumes)> {
        let vol_swept_c = self.V_swept_c;
        let vol_swept_e = self.V_swept_e;
        let vol_clear_c = self.V_clearance_c;
        let vol_clear_e = self.V_clearance_e;
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
        ThermalResistance {
            comp: self.R_c,
            exp: self.R_e,
        }
    }

    fn parasitics(&self, _state: &super::State) -> Parasitics {
        Parasitics {
            comp: ParasiticPower {
                thermal: 0.0,
                mechanical: self.W_parasitic_c,
                electrical: 0.0,
            },
            exp: ParasiticPower {
                thermal: self.Q_parasitic_e,
                mechanical: self.W_parasitic_e,
                electrical: 0.0,
            },
        }
    }

    fn report(&self, _state: &super::State) -> String {
        "Sinusoidal drive working spaces".to_string()
    }
}
