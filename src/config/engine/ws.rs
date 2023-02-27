use std::f64::INFINITY;

use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type", content = "params")]
pub(super) enum WorkingSpaces {
    Sinusoidal(Sinusoidal),
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct Sinusoidal {
    frequency: f64,
    phase_angle: f64,
    V_swept_c: f64,
    V_clearance_c: f64,
    R_c: f64,
    W_parasitic_c: f64,
    V_swept_e: f64,
    V_clearance_e: f64,
    R_e: f64,
    W_parasitic_e: f64,
    Q_parasitic_e: f64,
}

impl Default for Sinusoidal {
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

#[cfg(test)]
mod test {
    use super::WorkingSpaces;

    #[track_caller]
    fn check_ws(toml_str: &str, expected_ws: WorkingSpaces) {
        let settings = config::Config::builder()
            .add_source(config::File::from_str(toml_str, config::FileFormat::Toml))
            .build()
            .unwrap();
        assert_eq!(
            settings.try_deserialize::<WorkingSpaces>().unwrap(),
            expected_ws
        );
    }

    #[test]
    fn deserializing_sinusoidal() {
        check_ws(
            r#"
            type = "Sinusoidal"

            [params]
            frequency = 66.6667
            phase_angle = 90
            V_swept_c = 1.128e-4
            V_clearance_c = 4.68e-5
            R_c = inf
            W_parasitic_c = 0
            V_swept_e = 1.128e-4
            V_clearance_e = 1.68e-5
            R_e = inf
            W_parasitic_e = 0
            Q_parasitic_e = 0
            "#,
            WorkingSpaces::Sinusoidal(Default::default()),
        );
    }
}
