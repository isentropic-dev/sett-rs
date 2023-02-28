use serde::Deserialize;

use crate::{
    chx::ColdHeatExchangerConfig, fluid::FluidConfig, hhx::HotHeatExchangerConfig,
    regen::RegeneratorConfig, ws::WorkingSpacesConfig,
};

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct Engine {
    pub(crate) fluid: FluidConfig,
    pub(crate) components: Components,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct Components {
    pub(crate) chx: ColdHeatExchangerConfig,
    pub(crate) hhx: HotHeatExchangerConfig,
    pub(crate) regen: RegeneratorConfig,
    pub(crate) ws: WorkingSpacesConfig,
}

#[cfg(test)]
mod test {
    use crate::{
        chx::ColdHeatExchangerConfig,
        fluid::{FluidConfig, FluidModelConfig},
        hhx::HotHeatExchangerConfig,
        regen::RegeneratorConfig,
        ws::WorkingSpacesConfig,
    };

    use super::{Components, Engine};

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
            name = "hydrogen"
            model = "ideal_gas"

            [components.chx]
            type = "fixed_approach"
            vol = 4e-5
            DT = 40
            R_hyd = 0
            W_parasitic = 0

            [components.hhx]
            type = "fixed_approach"
            vol = 1e-4
            DT = 100
            R_hyd = 0
            W_parasitic = 0
            Q_parasitic = 0

            [components.regen]
            type = "fixed_approach"
            vol = 1e-4
            DT = 10
            R_hyd = 0
            Q_parasitic = 0

            [components.ws]
            type = "sinusoidal"
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
                fluid: FluidConfig::Hydrogen(FluidModelConfig::IdealGas),
                components: Components {
                    chx: ColdHeatExchangerConfig::FixedApproach(Default::default()),
                    hhx: HotHeatExchangerConfig::FixedApproach(Default::default()),
                    regen: RegeneratorConfig::FixedApproach(Default::default()),
                    ws: WorkingSpacesConfig::Sinusoidal(Default::default()),
                },
            },
        )
    }
}
