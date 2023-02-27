use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "name", content = "model")]
pub(crate) enum Fluid {
    Hydrogen(HydrogenModel),
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HydrogenModel {
    IdealGas,
}

#[cfg(test)]
mod test {
    use super::{Fluid, HydrogenModel};

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
            name = "hydrogen"
            model = "ideal_gas"
            "#,
            Fluid::Hydrogen(HydrogenModel::IdealGas),
        )
    }
}
