use std::{cell::RefCell, marker::PhantomData};

use anyhow::Result;
use ode_solvers::{Dopri5, System, Vector3};

use super::{
    solver::solve,
    types::{Conditions, FlowDirection, Time, Values},
    MatrixDecomposition, StateEquations,
};

/// Integrate state equations over a cycle
pub fn integrate<D: MatrixDecomposition, E: StateEquations>(
    engine: &E,
    period: Time,
    initial_conditions: Conditions,
    options: IntegrationOptions,
) -> Result<Vec<Values>> {
    let cycle: Cycle<D, E> = Cycle::new(engine, period);
    cycle.integrate(initial_conditions, options)
}

struct Cycle<'a, D: MatrixDecomposition, E: StateEquations> {
    decomp: PhantomData<D>,
    engine: &'a E,
    period: Time,
    last_flow_dir: RefCell<FlowDirection>,
    values: Vec<Values>,
}

impl<'a, D: MatrixDecomposition, E: StateEquations> Cycle<'a, D, E> {
    /// Create a `Cycle`
    fn new(engine: &'a E, period: Time) -> Self {
        Self {
            decomp: PhantomData,
            engine,
            period,
            last_flow_dir: RefCell::new(FlowDirection::default()),
            values: vec![],
        }
    }

    /// Integrate the state equations to the end of the `Cycle`
    fn integrate(
        &self,
        initial_conditions: Conditions,
        options: IntegrationOptions,
    ) -> Result<Vec<Values>> {
        let y0 = StateVariables::new(
            initial_conditions.P,
            initial_conditions.T_c,
            initial_conditions.T_e,
        );
        let mut stepper = Dopri5::new(
            self,
            0.0,                      // t_initial
            self.period + options.dx, // just past t_final
            options.dx,
            y0,
            options.rtol,
            options.atol,
        );
        stepper.integrate()?;

        let mut values = vec![]; // TODO: use with capacity
        let mut flow_dir = FlowDirection::default();
        for (&time, variables) in stepper.x_out().iter().zip(stepper.y_out().iter()) {
            let conditions = Conditions {
                P: variables[0],
                T_c: variables[1],
                T_e: variables[2],
            };
            let inputs = self.engine.calculate_inputs(time, conditions);
            let solution = solve::<D>(inputs, flow_dir)?;
            flow_dir = FlowDirection::from_solution(&solution);
            values.push(Values {
                time,
                conditions,
                solution,
            });
        }

        Ok(values)
    }
}

pub struct IntegrationOptions {
    dx: f64,
    rtol: f64,
    atol: f64,
}

impl IntegrationOptions {
    pub fn new(dx: f64, rtol: f64, atol: f64) -> Self {
        Self { dx, rtol, atol }
    }

    pub fn from_step_size(dx: f64) -> Self {
        Self {
            dx,
            rtol: 1e-6,
            atol: 1e-6,
        }
    }
}

type StateVariables = Vector3<f64>;

impl<D: MatrixDecomposition, E: StateEquations> System<StateVariables> for &Cycle<'_, D, E> {
    fn system(&self, time: Time, y: &StateVariables, dy: &mut StateVariables) {
        let conditions = Conditions {
            P: y[0],
            T_c: y[1],
            T_e: y[2],
        };
        let inputs = self.engine.calculate_inputs(time, conditions);
        let flow_dir = *self.last_flow_dir.borrow();
        let solution = solve::<D>(inputs, flow_dir).expect("TODO: what do we do if this fails?");

        let new_flow_dir = FlowDirection::from_solution(&solution);
        self.last_flow_dir.replace(new_flow_dir);

        dy[0] = solution.dP_dt;
        dy[1] = solution.dTc_dt;
        dy[2] = solution.dTe_dt;
    }
}
