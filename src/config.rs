use serde::Deserialize;

use crate::{
    engine,
    types::{ConditionsConfig, SolverConfig},
};

#[derive(Debug, Deserialize, PartialEq)]
struct Config {
    engine: engine::Config,
    solver: SolverConfig,
    conditions: ConditionsConfig,
}

#[cfg(test)]
mod test {
    use crate::{
        chx, engine, fluid, hhx, regen,
        types::{
            ConditionsConfig, InnerLoopConfig, OdeConfig, OuterLoopConfig, SolverConfig,
            ToleranceConfig,
        },
        ws,
    };

    use super::Config;

    #[track_caller]
    fn check_config(toml_str: &str, expected_config: Config) {
        let settings = config::Config::builder()
            .add_source(config::File::from_str(toml_str, config::FileFormat::Toml))
            .build()
            .unwrap();
        assert_eq!(
            settings.try_deserialize::<Config>().unwrap(),
            expected_config
        );
    }

    #[test]
    fn deserialize_config() {
        check_config(
            r#"
            [engine.fluid.hydrogen]
            model = "ideal_gas"

            [engine.components.chx.fixed_approach]
            vol = 4e-5
            DT = 40
            R_hyd = 0
            W_parasitic = 0

            [engine.components.hhx.fixed_approach]
            vol = 1e-4
            DT = 100
            R_hyd = 0
            W_parasitic = 0
            Q_parasitic = 0

            [engine.components.regen.fixed_approach]
            vol = 1e-4
            DT = 10
            R_hyd = 0
            Q_parasitic = 0

            [engine.components.ws.sinusoidal]
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
            
            [solver.inner_loop]
            tolerance = { abs = 1e-6, rel = 1e-6 }
            max_iterations = 10

            [solver.outer_loop]
            tolerance = { abs = 1e-8, rel = 1e-8 }
            max_iterations = 10

            [solver.ode]
            tolerance = { abs = 1e-8, rel = 1e-8 }
            num_timesteps = 20

            [conditions]
            T_cold = 20
            T_hot = 50
            P_0 = 100
            "#,
            Config {
                engine: engine::Config {
                    fluid: fluid::Config::Hydrogen(fluid::ModelConfig::IdealGas),
                    components: engine::ComponentsConfig {
                        chx: chx::Config::FixedApproach(Default::default()),
                        hhx: hhx::Config::FixedApproach(Default::default()),
                        regen: regen::Config::FixedApproach(Default::default()),
                        ws: ws::Config::Sinusoidal(Default::default()),
                    },
                },
                solver: SolverConfig {
                    inner_loop: InnerLoopConfig {
                        tolerance: ToleranceConfig {
                            abs: 1e-6,
                            rel: 1e-6,
                        },
                        max_iterations: 10,
                    },
                    outer_loop: OuterLoopConfig {
                        tolerance: ToleranceConfig {
                            abs: 1e-8,
                            rel: 1e-8,
                        },
                        max_iterations: 10,
                    },
                    ode: OdeConfig {
                        tolerance: ToleranceConfig {
                            abs: 1e-8,
                            rel: 1e-8,
                        },
                        num_timesteps: 20,
                    },
                },
                conditions: ConditionsConfig {
                    T_cold: 20.,
                    T_hot: 50.,
                    P_0: 100.,
                },
            },
        )
    }
}
