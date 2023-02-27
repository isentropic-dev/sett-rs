use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type", content = "params")]
pub(crate) enum ColdHeatExchanger {
    FixedApproach(CHXFixedApproach),
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct CHXFixedApproach {
    pub(crate) vol: f64,
    pub(crate) DT: f64,
    pub(crate) R_hyd: f64,
    pub(crate) W_parasitic: f64,
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

#[cfg(test)]
mod test {
    use super::ColdHeatExchanger;

    #[track_caller]
    fn check_chx(toml_str: &str, expected_chx: ColdHeatExchanger) {
        let settings = config::Config::builder()
            .add_source(config::File::from_str(toml_str, config::FileFormat::Toml))
            .build()
            .unwrap();
        assert_eq!(
            settings.try_deserialize::<ColdHeatExchanger>().unwrap(),
            expected_chx
        );
    }

    #[test]
    fn deserializing_fixed_approach() {
        check_chx(
            r#"
            type = "FixedApproach"

            [params]
            vol = 4e-5
            DT = 40
            R_hyd = 0
            W_parasitic = 0
            "#,
            ColdHeatExchanger::FixedApproach(Default::default()),
        );
    }
}
