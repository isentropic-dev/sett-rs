use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct Solver {
    pub(crate) inner_loop: InnerLoop,
    pub(crate) outer_loop: OuterLoop,
    pub(crate) ode: OrdinaryDifferentialEquation,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct InnerLoop {
    pub(crate) tolerance: Tolerance,
    pub(crate) max_iterations: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct OuterLoop {
    pub(crate) tolerance: Tolerance,
    pub(crate) max_iterations: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct OrdinaryDifferentialEquation {
    pub(crate) tolerance: Tolerance,
    pub(crate) num_timesteps: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct Tolerance {
    pub(crate) abs: f64,
    pub(crate) rel: f64,
}

#[cfg(test)]
mod test {
    use super::{InnerLoop, OrdinaryDifferentialEquation, OuterLoop, Solver, Tolerance};

    #[track_caller]
    fn check_solver(toml_str: &str, expected_solver: Solver) {
        let settings = config::Config::builder()
            .add_source(config::File::from_str(toml_str, config::FileFormat::Toml))
            .build()
            .unwrap();
        assert_eq!(
            settings.try_deserialize::<Solver>().unwrap(),
            expected_solver
        );
    }

    #[test]
    fn deserializing_solver() {
        check_solver(
            r#"
            [inner_loop]
            tolerance = { abs = 1e-6, rel = 1e-6 }
            max_iterations = 10

            [outer_loop]
            tolerance = { abs = 1e-8, rel = 1e-8 }
            max_iterations = 10

            [ode]
            tolerance = { abs = 1e-8, rel = 1e-8 }
            num_timesteps = 20
            "#,
            Solver {
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
        )
    }
}
