use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct Conditions {
    pub(crate) T_cold: f64,
    pub(crate) T_hot: f64,
    pub(crate) P_0: f64,
}

#[cfg(test)]
mod test {

    use super::Conditions;

    #[track_caller]
    fn check_conditions(toml_str: &str, expected_conditions: Conditions) {
        let settings = config::Config::builder()
            .add_source(config::File::from_str(toml_str, config::FileFormat::Toml))
            .build()
            .unwrap();
        assert_eq!(
            settings.try_deserialize::<Conditions>().unwrap(),
            expected_conditions
        );
    }

    #[test]
    fn deserializing_conditions() {
        check_conditions(
            r#"
            T_cold = 20
            T_hot = 50
            P_0 = 100
            "#,
            Conditions {
                T_cold: 20.,
                T_hot: 50.,
                P_0: 100.,
            },
        );
    }
}
