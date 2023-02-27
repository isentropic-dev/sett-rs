use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "model", content = "params")]
pub enum Fluid {
    IdealGas { name: FluidName },
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum FluidName {
    Hydrogen,
}

#[cfg(test)]
mod test {
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
    fn deserializing_fluid() {
        check_fluid(
            r#"
            model = "IdealGas"
            params = { name = "Hydrogen"}
            "#,
            Fluid::IdealGas {
                name: FluidName::Hydrogen,
            },
        )
    }
}
