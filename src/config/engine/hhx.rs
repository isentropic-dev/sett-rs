use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type", content = "params")]
pub(super) enum HotHeatExchanger {
    FixedApproach(HHXFixedApproach),
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct HHXFixedApproach {
    vol: f64,
    DT: f64,
    R_hyd: f64,
    W_parasitic: f64,
    Q_parasitic: f64,
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

#[cfg(test)]
mod test {
    use super::HotHeatExchanger;

    #[track_caller]
    fn check_hhx(toml_str: &str, expected_hhx: HotHeatExchanger) {
        let settings = config::Config::builder()
            .add_source(config::File::from_str(toml_str, config::FileFormat::Toml))
            .build()
            .unwrap();
        assert_eq!(
            settings.try_deserialize::<HotHeatExchanger>().unwrap(),
            expected_hhx
        );
    }

    #[test]
    fn deserializing_fixed_approach() {
        check_hhx(
            r#"
            type = "FixedApproach"

            [params]
            vol = 1e-4
            DT = 100
            R_hyd = 0
            W_parasitic = 0
            Q_parasitic = 0
            "#,
            HotHeatExchanger::FixedApproach(Default::default()),
        );
    }
}
