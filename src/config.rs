mod conditions;
mod engine;
mod solver;

use serde::Deserialize;

use crate::config::{conditions::Conditions, engine::Engine, solver::Solver};

#[derive(Debug, Deserialize, PartialEq)]
struct Config {
    engine: Engine,
    solver: Solver,
    conditions: Conditions,
}

#[cfg(test)]
mod test {
    use crate::{
        chx::ColdHeatExchangerConfig, hhx::HotHeatExchangerConfig, regen::RegeneratorConfig,
        ws::WorkingSpacesConfig,
    };

    use super::{
        conditions::Conditions,
        engine::{
            fluid::{Fluid, HydrogenModel},
            Components, Engine,
        },
        solver::{InnerLoop, OrdinaryDifferentialEquation, OuterLoop, Solver, Tolerance},
        Config,
    };

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
            [engine.fluid]
            name = "hydrogen"
            model = "ideal_gas"

            [engine.components.chx]
            type = "fixed_approach"
            vol = 4e-5
            DT = 40
            R_hyd = 0
            W_parasitic = 0

            [engine.components.hhx]
            type = "fixed_approach"
            vol = 1e-4
            DT = 100
            R_hyd = 0
            W_parasitic = 0
            Q_parasitic = 0

            [engine.components.regen]
            type = "fixed_approach"
            vol = 1e-4
            DT = 10
            R_hyd = 0
            Q_parasitic = 0

            [engine.components.ws]
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
                engine: Engine {
                    fluid: Fluid::Hydrogen(HydrogenModel::IdealGas),
                    components: Components {
                        chx: ColdHeatExchangerConfig::FixedApproach(Default::default()),
                        hhx: HotHeatExchangerConfig::FixedApproach(Default::default()),
                        regen: RegeneratorConfig::FixedApproach(Default::default()),
                        ws: WorkingSpacesConfig::Sinusoidal(Default::default()),
                    },
                },
                solver: Solver {
                    inner_loop: InnerLoop {
                        tolerance: Tolerance {
                            abs: 1e-6_f64,
                            rel: 1e-6_f64,
                        },
                        max_iterations: 10,
                    },
                    outer_loop: OuterLoop {
                        tolerance: Tolerance {
                            abs: 1e-8_f64,
                            rel: 1e-8_f64,
                        },
                        max_iterations: 10,
                    },
                    ode: OrdinaryDifferentialEquation {
                        tolerance: Tolerance {
                            abs: 1e-8_f64,
                            rel: 1e-8_f64,
                        },
                        num_timesteps: 20,
                    },
                },
                conditions: Conditions {
                    T_cold: 20.,
                    T_hot: 50.,
                    P_0: 100.,
                },
            },
        )
    }
}
