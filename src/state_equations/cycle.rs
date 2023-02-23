use anyhow::{bail, Result};

use crate::types::{ConvergenceTolerance, OdeTolerance};

use super::{integrator::Integration, Conditions, Inputs, MatrixDecomposition, Values};

pub trait Cycle: Sized {
    type Solver: MatrixDecomposition;

    /// Calculate the inputs to the state equations
    fn calculate_inputs(&self, time: f64, conditions: Conditions) -> Inputs;

    /// Return the period in seconds for the cycle
    fn period(&self) -> f64;

    /// Return the pressure in Pa at time zero in the cycle
    fn pres_zero(&self) -> f64;

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
    /// Cyclic steady state occurs when the temperature conditions (`T_c` and
    /// `T_e`) at the end of the cycle are equal to those at the start.
    fn find_steady_state(&self, inputs: SteadyStateInputs) -> Result<Vec<Values>> {
        let SteadyStateInputs {
            pres_zero,
            temp_comp_hint,
            temp_exp_hint,
            num_points,
            ode_tol,
            conv_tol,
            max_iters,
        } = inputs;

        // Use successive substition to find the right initial conditions
        // TODO: Create an abstraction of this so we can try different convergence methods
        let mut ic = Conditions {
            P: pres_zero,
            T_c: temp_comp_hint,
            T_e: temp_exp_hint,
        };
        for _ in 0..max_iters {
            let integration = Integration::try_from(self, ic, 2, ode_tol)?; // using 2 points here is faster and doesn't affect integration
            if integration.is_converged(conv_tol) {
                let integration = Integration::try_from(self, ic, num_points, ode_tol)?;
                return Ok(integration.into_state_values());
            }
            ic = Conditions {
                P: pres_zero,
                ..integration.final_conditions() // succesive substitution
            };
        }

        bail!("did not converge")
    }
}

pub struct SteadyStateInputs {
    pub pres_zero: f64,
    pub temp_comp_hint: f64,
    pub temp_exp_hint: f64,
    pub num_points: u32,
    pub ode_tol: OdeTolerance,
    pub conv_tol: ConvergenceTolerance,
    pub max_iters: usize,
}
