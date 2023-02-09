use std::cell::RefCell;

use anyhow::Result;
use ode_solvers::{Dopri5, System, Vector3};

use super::{
    solver::solve,
    types::{Conditions, FlowDirection, StateValues, Time},
    Cycle,
};

/// Represents an integration of the state equations over a cycle
pub struct Integration<'a, T: Cycle> {
    cycle: &'a T,
    points: Vec<Point>,
}

impl<'a, T: Cycle> Integration<'a, T> {
    /// Attempt to integrate the state equations
    pub fn try_from(
        cycle: &'a T,
        initial_conditions: Conditions,
        num_points: u32,
        tol: OdeTolerance,
    ) -> Result<Self> {
        let state = IntegrationState {
            cycle,
            last_flow_dir: RefCell::new(FlowDirection::default()),
        };
        let period = cycle.period();
        let dx = period / f64::from(num_points - 1);
        let y0 = StateVariables::new(
            initial_conditions.P,
            initial_conditions.T_c,
            initial_conditions.T_e,
        );
        let mut stepper = Dopri5::new(
            state,
            0.0,         // t_initial
            period + dx, // just past t_final
            dx,
            y0,
            tol.rel,
            tol.abs,
        );
        stepper.integrate()?;

        let mut points = Vec::with_capacity(num_points as usize);
        for (&time, variables) in stepper.x_out().iter().zip(stepper.y_out().iter()) {
            let conditions = Conditions {
                P: variables[0],
                T_c: variables[1],
                T_e: variables[2],
            };
            points.push(Point { time, conditions });
        }

        Ok(Self { cycle, points })
    }

    /// Check if the integration over the cycle is converged
    ///
    /// A converged integration is one where the temperatures in the
    /// compression (`T_c`) and expansion (`T_e`) spaces at the end of
    /// the cycle are equal to their respective initial conditions.  Only
    /// the temperatures are checked because pressure must converge due to
    /// conservation of mass and energy in the state equations.
    pub fn is_converged(&self, tol: ConvergenceTolerance) -> bool {
        let first_point = self.points.first().unwrap(); // `self.points` is never empty
        let last_point = self.points.last().unwrap();
        tol.is_converged(first_point.conditions.T_c, last_point.conditions.T_c)
            && tol.is_converged(first_point.conditions.T_e, last_point.conditions.T_e)
    }

    /// Return the final time of the integration
    pub fn final_time(&self) -> f64 {
        let last_point = self.points.last().unwrap(); // `self.points` is never empty
        last_point.time
    }

    /// Calculate state equation values at each point
    ///
    /// This function consumes the `Integration`.
    pub fn into_state_values(self) -> Vec<StateValues> {
        let mut flow_dir = FlowDirection::default();
        self.points
            .into_iter()
            .map(|point| {
                let Point { time, conditions } = point;
                let inputs = self.cycle.calculate_inputs(time, conditions);
                let solution = solve::<T::Solver>(inputs, flow_dir)
                    .expect("TODO: what do we do if this fails?");
                flow_dir = FlowDirection::from_solution(&solution);
                StateValues {
                    time,
                    conditions,
                    solution,
                }
            })
            .collect()
    }
}

/// Tolerances used by the ODE integrator
#[derive(Debug, Clone, Copy)]
pub struct OdeTolerance {
    abs: f64,
    rel: f64,
}

impl OdeTolerance {
    pub fn new(abs: f64, rel: f64) -> Self {
        Self { abs, rel }
    }
}

/// Tolerances related to convergence between subsequent values
#[derive(Debug, Clone, Copy)]
pub struct ConvergenceTolerance {
    abs: f64,
    rel: f64,
}

impl ConvergenceTolerance {
    pub fn new(abs: f64, rel: f64) -> Self {
        Self { abs, rel }
    }

    /// Return `true` if the change from `old` to `new` is sufficiently small
    fn is_converged(&self, old: f64, new: f64) -> bool {
        let abs_change = new - old;
        let rel_change = abs_change / old;
        abs_change.abs() < self.abs && rel_change.abs() < self.rel
    }
}

/// The conditions within the cycle at a point in time
struct Point {
    time: Time,
    conditions: Conditions,
}

/// The variables being integrated
///
/// The order is [`P`, `T_c`, `T_e`].
type StateVariables = Vector3<f64>;

struct IntegrationState<'a, T: Cycle> {
    cycle: &'a T,
    last_flow_dir: RefCell<FlowDirection>,
}

impl<T: Cycle> System<StateVariables> for IntegrationState<'_, T> {
    fn system(&self, time: f64, y: &StateVariables, dy: &mut StateVariables) {
        let conditions = Conditions {
            P: y[0],
            T_c: y[1],
            T_e: y[2],
        };
        let inputs = self.cycle.calculate_inputs(time, conditions);
        let flow_dir_hint = *self.last_flow_dir.borrow();
        let solution = solve::<T::Solver>(inputs, flow_dir_hint)
            .expect("TODO: what should we do if this fails?");

        let flow_dir = FlowDirection::from_solution(&solution);
        self.last_flow_dir.replace(flow_dir);

        dy[0] = solution.dP_dt;
        dy[1] = solution.dTc_dt;
        dy[2] = solution.dTe_dt;
    }
}
