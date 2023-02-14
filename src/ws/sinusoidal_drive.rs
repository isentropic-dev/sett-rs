use std::f64::consts::PI;

use super::{Parasitics, ThermalResistance, Volumes, WorkingSpaces};

#[allow(non_snake_case)]
pub struct SinusoidalDrive {
    frequency: f64,
    phase_angle: f64,
    comp_geometry: Geometry,
    exp_geometry: Geometry,
    thermal_resistance: ThermalResistance,
    parasitics: Parasitics,
}

impl SinusoidalDrive {
    /// Initialize the builder for a `SinusoidalDrive`
    pub fn builder() -> Builder {
        Builder {
            frequency: None,
            phase_angle: None,
            comp_geometry: None,
            exp_geometry: None,
            thermal_resistance: ThermalResistance::default(), // default conditions are adiabatic
            parasitics: Parasitics::default(),                // default is no parasitic losses
        }
    }

    /// Create a `SinusoidalDrive` component from the legacy JSON format
    ///
    /// The legacy JSON format for the sinusoidal drive component used by the
    /// MATLAB version of SETT has the following format:
    ///
    /// ```json
    /// {
    ///    "frequency", 66.6667,
    ///    "phaseAngle", 90,
    ///    "V_swept_c", 1.1e-4,
    ///    "V_clearance_c", 4.7e-5,
    ///    "V_swept_e", 1.1e-4,
    ///    "V_clearance_e", 1.7e-5,
    ///    "R_c", "Inf",
    ///    "R_e", "Inf",
    ///    "W_parasitic_c", 0,
    ///    "W_parasitic_e", 0,
    ///    "Q_parasitic_e", 0
    /// }
    pub fn from_legacy_json() -> Self {
        todo!()
    }
}

pub struct Builder {
    frequency: Option<f64>,
    phase_angle: Option<f64>,
    comp_geometry: Option<Geometry>,
    exp_geometry: Option<Geometry>,
    thermal_resistance: ThermalResistance,
    parasitics: Parasitics,
}

impl Builder {
    /// Convert the builder into a `SinusoidalDrive`
    pub fn build(self) -> SinusoidalDrive {
        let frequency = self
            .frequency
            .expect("frequency must be set using the `with_frequency()` method");
        let phase_angle = self
            .phase_angle
            .expect("phase angle must be set using the `with_phase_angle()` method");
        let comp_geometry = self.comp_geometry.expect(
            "compression space volumes must be set using the `with_compression_volumes()` method",
        );
        let exp_geometry = self.exp_geometry.expect(
            "expansion space volumes must be set using the `with_expansion_volumes()` method",
        );
        SinusoidalDrive {
            frequency,
            phase_angle,
            comp_geometry,
            exp_geometry,
            thermal_resistance: self.thermal_resistance,
            parasitics: self.parasitics,
        }
    }

    /// Set the `frequency` value in Hz
    pub fn with_frequency(self, frequency: f64) -> Self {
        Self {
            frequency: Some(frequency),
            ..self
        }
    }

    /// Set the `phase_angle` value in degrees
    pub fn with_phase_angle(self, phase_angle: f64) -> Self {
        Self {
            phase_angle: Some(phase_angle),
            ..self
        }
    }

    /// Define the compression space geometry in m^3
    pub fn with_compression_volumes(self, clearance: f64, swept: f64) -> Self {
        Self {
            comp_geometry: Some(Geometry {
                clearance_volume: clearance,
                swept_volume: swept,
            }),
            ..self
        }
    }

    /// Define the expansion space geometry in m^3
    pub fn with_expansion_volumes(self, clearance: f64, swept: f64) -> Self {
        Self {
            exp_geometry: Some(Geometry {
                clearance_volume: clearance,
                swept_volume: swept,
            }),
            ..self
        }
    }

    /// Set the `thermal_resistance` value
    pub fn with_thermal_resistance(self, thermal_resistance: ThermalResistance) -> Self {
        Self {
            thermal_resistance,
            ..self
        }
    }

    /// Set the `parasitics` value
    pub fn with_parasitics(self, parasitics: Parasitics) -> Self {
        Self { parasitics, ..self }
    }
}

struct Geometry {
    clearance_volume: f64,
    swept_volume: f64,
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
        let drive = SinusoidalDrive::builder()
            .with_frequency(10.0)
            .with_phase_angle(90.0)
            .with_compression_volumes(clear_vol_c, swept_vol_c)
            .with_expansion_volumes(clear_vol_e, swept_vol_e)
            .build();
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
