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

#[derive(Debug, Deserialize, PartialEq)]
struct ColdHeatExchanger {}
#[derive(Debug, Deserialize, PartialEq)]
struct HotHeatExchanger {}
#[derive(Debug, Deserialize, PartialEq)]
struct Regenerator {}
#[derive(Debug, Deserialize, PartialEq)]
struct WorkingSpaces {}

#[cfg(test)]
mod tests {
    use super::{Fluid, FluidName};

    #[track_caller]
    fn check_fluid(toml_str: &str, expected_fluid: Fluid) {
        let settings = config::Config::builder()
            .add_source(config::File::from_str(toml_str, config::FileFormat::Toml))
            .build()
            .unwrap();
        assert_eq!(settings.try_deserialize::<Fluid>().unwrap(), expected_fluid);
    }

    #[test]
    fn deserializing_a_fluid() {
        check_fluid(
            r#"
            model = "IdealGas"

            [params]
            name = "Hydrogen"
            "#,
            Fluid::IdealGas {
                name: FluidName::Hydrogen,
            },
        )
    }
}
