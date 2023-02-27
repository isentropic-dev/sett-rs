use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub(super) struct Solver {
    inner_loop: InnerLoop,
    outer_loop: OuterLoop,
    ode: OrdinaryDifferentialEquation,
}

#[derive(Debug, Deserialize, PartialEq)]
struct InnerLoop {
    tolerance: Tolerance,
    max_iterations: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
struct OuterLoop {
    tolerance: Tolerance,
    max_iterations: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
struct OrdinaryDifferentialEquation {
    tolerance: Tolerance,
    num_timesteps: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Tolerance {
    abs: f64,
    rel: f64,
}

#[cfg(test)]
mod test {
    use super::Solver;

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
            r#""#,
            Solver {
                inner_loop: todo!(),
                outer_loop: todo!(),
                ode: todo!(),
            },
        )
    }
}
