mod cycle;
mod flow_direction;
mod inputs;
mod integrator;
mod solver;

use serde::Serialize;

// Export traits
pub use self::{cycle::Cycle, solver::MatrixDecomposition};

// Export input types
pub use self::{
    inputs::HeatExchanger as HeatExchangerInputs, inputs::Inputs,
    inputs::Regenerator as RegeneratorInputs, inputs::WorkingSpace as WorkingSpaceInputs,
};

// Export matrix decomposition solvers
pub use self::solver::{SvdDefault as SvdDefaultSolver, LU as LuSolver, QR as QrSolver};

/// Conditions within the cycle
///
/// `P`   -- pressure (Pa) in all control volumes
/// `T_c` -- temperature (K) in the compression space
/// `T_e` -- temperature (K) in the expansion space
#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Conditions {
    pub P: f64,
    pub T_c: f64,
    pub T_e: f64,
}

/// Represents a solution to the state equations
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize)]
pub struct Solution {
    pub m_dot_ck: f64,
    pub m_dot_kr: f64,
    pub m_dot_rl: f64,
    pub m_dot_le: f64,
    pub Q_dot_k: f64,
    pub Q_dot_r: f64,
    pub Q_dot_l: f64,
    pub dTc_dt: f64,
    pub dTe_dt: f64,
    pub dP_dt: f64,
}

/// The solution to the state equations for some conditions and time
#[derive(Debug, Clone, Serialize)]
pub struct Values {
    pub time: f64,
    pub conditions: Conditions,
    pub solution: Solution,
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use approx::assert_relative_eq;

    use crate::types::{ConvergenceTolerance, OdeTolerance};

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
        type Solver = solver::LU;

        fn calculate_inputs(&self, _time: f64, _conditions: Conditions) -> Inputs {
            self.inputs.clone()
        }

        fn period(&self) -> f64 {
            0.5
        }

        fn pres_zero(&self) -> f64 {
            1.0
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
        let integration = engine
            .integrate(initial_conditions, num_points, ode_tol)
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
        let integration = engine
            .integrate(initial_conditions, num_points, ode_tol)
            .expect("integration should work");

        assert_relative_eq!(integration.final_time(), engine.period(), epsilon = 1e-12);

        let values = integration.into_state_values();
        assert_eq!(values.len(), 101);
    }

    #[test]
    fn should_not_find_steady_state() {
        let engine = TestEngine::from_file("refprop_hydrogen.json");

        let num_points = 100;
        let ode_tol = OdeTolerance::new(1e-4, 1e-4);
        let conv_tol = ConvergenceTolerance::new(1e-4, 1e-4);
        let max_iter = 20;

        engine
            .find_steady_state(10e6, (300., 500.), num_points, ode_tol, conv_tol, max_iter)
            .expect_err("should not find steady state");
    }
}
