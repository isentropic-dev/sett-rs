mod integrator;
mod solver;
mod types;

pub use self::{
    integrator::{integrate, IntegrationOptions},
    solver::{MatrixDecomposition, SvdDefault, LU, QR},
    types::{Conditions, Inputs, Time},
};

pub trait StateEquations {
    /// Calculate the inputs to the state equations
    fn calculate_inputs(&self, time: Time, conditions: Conditions) -> Inputs;
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

    impl StateEquations for TestEngine {
        fn calculate_inputs(&self, _time: Time, _conditionss: Conditions) -> Inputs {
            self.inputs
        }
    }

    #[test]
    fn integrates_to_final_time() {
        let engine = TestEngine::from_file("ideal_gas_hydrogen.json");
        let period = 0.5;
        let initial_conditions = Conditions {
            P: 10e6,
            T_c: 400.0,
            T_e: 600.0,
        };
        let options = IntegrationOptions::from_step_size(0.01);
        let values = integrate::<LU, _>(&engine, period, initial_conditions, options)
            .expect("integration should work");
        let final_values = &values.last().expect("vector should not be empty");
        assert_relative_eq!(final_values.time, period, epsilon = 1e-12);
    }
}
