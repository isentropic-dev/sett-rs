use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type", content = "params")]
pub(crate) enum Regenerator {
    FixedApproach(RegenFixedApproach),
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct RegenFixedApproach {
    pub(crate) vol: f64,
    pub(crate) DT: f64,
    pub(crate) R_hyd: f64,
    pub(crate) Q_parasitic: f64,
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

#[cfg(test)]
mod test {
    use super::Regenerator;

    #[track_caller]
    fn check_regen(toml_str: &str, expected_regen: Regenerator) {
        let settings = config::Config::builder()
            .add_source(config::File::from_str(toml_str, config::FileFormat::Toml))
            .build()
            .unwrap();
        assert_eq!(
            settings.try_deserialize::<Regenerator>().unwrap(),
            expected_regen
        );
    }

    #[test]
    fn deserializing_fixed_approach() {
        check_regen(
            r#"
            type = "FixedApproach"

            [params]
            vol = 1e-4
            DT = 10
            R_hyd = 0
            Q_parasitic = 0
            "#,
            Regenerator::FixedApproach(Default::default()),
        );
    }
}
