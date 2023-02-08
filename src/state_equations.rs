mod integrator;
mod solver;
mod types;

pub use self::{
    integrator::{ConvergenceTolerance, Integration, OdeTolerance},
    solver::{SvdDefault, LU, QR},
    types::{Conditions, Inputs, StateValues, Time},
};

pub trait Cycle {
    type Solver: solver::MatrixDecomposition;

    /// Calculate the inputs to the state equations
    fn calculate_inputs(&self, time: Time, conditions: Conditions) -> Inputs;

    /// Return the period in seconds for the cycle
    fn period(&self) -> Time;
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use std::{fs, path::PathBuf};

    use super::*;

    /// Read a file containing test inputs related to state equations
    pub fn read_test_inputs(filename: &str) -> String {
        let file: PathBuf = [
            env!("CARGO_MANIFEST_DIR"),
            "src",
            "state_equations",
            "test_inputs",
            filename,
        ]
        .iter()
        .collect();
        fs::read_to_string(file).expect("test inputs file is missing")
    }

    struct TestEngine {
        inputs: Inputs,
    }

    impl TestEngine {
        fn from_file(filename: &str) -> Self {
            let inputs = read_test_inputs(filename);
            let inputs = serde_json::from_str(&inputs).expect("test inputs file is invalid");
            Self { inputs }
        }
    }

    impl Cycle for TestEngine {
        type Solver = LU;

        fn calculate_inputs(&self, _time: Time, _conditions: Conditions) -> Inputs {
            self.inputs.clone()
        }

        fn period(&self) -> Time {
            0.5
        }
    }

    #[test]
    fn integrates_to_final_time() {
        let engine = TestEngine::from_file("ideal_gas_hydrogen.json");
        let initial_conditions = Conditions {
            P: 10e6,
            T_c: 400.0,
            T_e: 600.0,
        };
        let num_points = 21;
        let ode_tol = OdeTolerance::new(1e-4, 1e-4);
        let integration = Integration::try_from(&engine, initial_conditions, num_points, ode_tol)
            .expect("integration should work");

        let conv_tol = ConvergenceTolerance::new(1e-4, 1e-4);
        assert!(
            !integration.is_converged(conv_tol),
            "integration should not be converged"
        );

        assert_relative_eq!(integration.final_time(), engine.period(), epsilon = 1e-12);
    }

    #[test]
    fn calculates_state_values() {
        let engine = TestEngine::from_file("refprop_hydrogen.json");
        let initial_conditions = Conditions {
            P: 10e6,
            T_c: 300.0,
            T_e: 500.0,
        };
        let num_points = 101;
        let ode_tol = OdeTolerance::new(1e-6, 1e-6);
        let integration = Integration::try_from(&engine, initial_conditions, num_points, ode_tol)
            .expect("integration should work");

        assert_relative_eq!(integration.final_time(), engine.period(), epsilon = 1e-12);

        let values = integration.into_state_values();
        assert_eq!(values.len(), 101);
    }
}
