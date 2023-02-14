use anyhow::{bail, Result};

use crate::types::{ConvergenceTolerance, OdeTolerance};

use super::{integrator::Integration, Conditions, Inputs, MatrixDecomposition, Values};

pub trait Cycle: Sized {
    type Solver: MatrixDecomposition;

    /// Calculate the inputs to the state equations
    fn calculate_inputs(&self, time: f64, conditions: Conditions) -> Inputs;

    /// Return the period in seconds for the cycle
    fn period(&self) -> f64;

    /// Attempt to integrate the state equations
    fn integrate(
        &self,
        initial_conditions: Conditions,
        num_points: u32,
        tol: OdeTolerance,
    ) -> Result<Integration<Self>> {
        Integration::try_from(self, initial_conditions, num_points, tol)
    }

    /// Determine the values that correspond to cyclic steady state
    ///
    /// Cyclic steady state is when the conditions (`P`, `T_c`, and `T_e`) at
    /// the end of the cycle are equal to those at the start.
    fn find_steady_state(
        &self,
        num_points: u32,
        ic_hint: Conditions,
        ode_tol: OdeTolerance,
        conv_tol: ConvergenceTolerance,
        max_iter: usize,
    ) -> Result<Vec<Values>> {
        // Use successive substition to find the right initial conditions
        // TODO: Create an abstraction of this so we can try different convergence methods
        let mut ic = ic_hint;
        for _ in 0..max_iter {
            let integration = Integration::try_from(self, ic, 2, ode_tol)?; // using 2 points here is faster and doesn't affect integration
            if integration.is_converged(conv_tol) {
                let integration = Integration::try_from(self, ic, num_points, ode_tol)?;
                return Ok(integration.into_state_values());
            }
            ic = integration.final_conditions(); // successive substitution
        }

        bail!("did not converge")
    }
}
