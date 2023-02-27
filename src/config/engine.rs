use std::f64::INFINITY;

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct Engine {
    fluid: Fluid,
    components: Components,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "model", content = "params")]
enum Fluid {
    IdealGas { name: FluidName },
}

#[derive(Debug, Deserialize, PartialEq)]
enum FluidName {
    Hydrogen,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Components {
    chx: ColdHeatExchanger,
    hhx: HotHeatExchanger,
    regen: Regenerator,
    ws: WorkingSpaces,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type", content = "params")]
enum ColdHeatExchanger {
    FixedApproach(CHXFixedApproach),
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
struct CHXFixedApproach {
    vol: f64,
    DT: f64,
    R_hyd: f64,
    W_parasitic: f64,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type", content = "params")]
enum HotHeatExchanger {
    FixedApproach(HHXFixedApproach),
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
struct HHXFixedApproach {
    vol: f64,
    DT: f64,
    R_hyd: f64,
    W_parasitic: f64,
    Q_parasitic: f64,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type", content = "params")]
enum Regenerator {
    FixedApproach(RegenFixedApproach),
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
struct RegenFixedApproach {
    vol: f64,
    DT: f64,
    R_hyd: f64,
    Q_parasitic: f64,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type", content = "params")]
enum WorkingSpaces {
    Sinusoidal(Sinusoidal),
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
struct Sinusoidal {
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

impl Default for CHXFixedApproach {
    fn default() -> Self {
        Self {
            vol: 4e-5_f64,
            DT: 40.,
            R_hyd: 0.,
            W_parasitic: 0.,
        }
    }
}
impl Default for HHXFixedApproach {
    fn default() -> Self {
        Self {
            vol: 1e-4_f64,
            DT: 100.,
            R_hyd: 0.,
            W_parasitic: 0.,
            Q_parasitic: 0.,
        }
    }
}
impl Default for RegenFixedApproach {
    fn default() -> Self {
        Self {
            vol: 1e-4_f64,
            DT: 10.,
            R_hyd: 0.,
            Q_parasitic: 0.,
        }
    }
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
mod tests {
    use super::{
        ColdHeatExchanger, Components, Engine, Fluid, FluidName, HotHeatExchanger, Regenerator,
        WorkingSpaces,
    };

    #[track_caller]
    fn check_engine(toml_str: &str, expected_engine: Engine) {
        let settings = config::Config::builder()
            .add_source(config::File::from_str(toml_str, config::FileFormat::Toml))
            .build()
            .unwrap();
        assert_eq!(
            settings.try_deserialize::<Engine>().unwrap(),
            expected_engine
        );
    }

    #[test]
    fn deserializing_an_engine() {
        check_engine(
            r#"
            [fluid]
            model = "IdealGas"
            params = { name = "Hydrogen"}

            [components.chx]
            type = "FixedApproach"

            [components.chx.params]
            vol = 4e-5
            DT = 40
            R_hyd = 0
            W_parasitic = 0

            [components.hhx]
            type = "FixedApproach"

            [components.hhx.params]
            vol = 1e-4
            DT = 100
            R_hyd = 0
            W_parasitic = 0
            Q_parasitic = 0

            [components.regen]
            type = "FixedApproach"

            [components.regen.params]
            vol = 1e-4
            DT = 10
            R_hyd = 0
            Q_parasitic = 0

            [components.ws]
            type = "Sinusoidal"

            [components.ws.params]
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
            Engine {
                fluid: Fluid::IdealGas {
                    name: FluidName::Hydrogen,
                },
                components: Components {
                    chx: ColdHeatExchanger::FixedApproach(Default::default()),
                    hhx: HotHeatExchanger::FixedApproach(Default::default()),
                    regen: Regenerator::FixedApproach(Default::default()),
                    ws: WorkingSpaces::Sinusoidal(Default::default()),
                },
            },
        )
    }
}
