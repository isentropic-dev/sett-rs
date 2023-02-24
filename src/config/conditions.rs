use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct Conditions {
    T_cold: f64,
    T_hot: f64,
    P_0: f64,
}

#[cfg(test)]
mod test {
    use config::Config;

    use super::Conditions;

    #[test]
    fn deserializing_conditions() {
        let settings = Config::builder()
            .add_source(config::File::from_str(
                r#"
            T_cold = 20
            T_hot = 50
            P_0 = 100
            "#,
                config::FileFormat::Toml,
            ))
            .build()
            .unwrap();

        let conditions = settings.try_deserialize::<Conditions>().unwrap();
        assert_eq!(
            conditions,
            Conditions {
                T_cold: 20.,
                T_hot: 50.,
                P_0: 100.
            }
        );
    }
}
