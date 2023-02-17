use std::marker::PhantomData;

use anyhow::{bail, Result};

use crate::{
    fluid::Fluid,
    state_equations::{
        Conditions, Cycle, HeatExchangerInputs, Inputs as StateEquationInputs, MatrixDecomposition,
        RegeneratorInputs, WorkingSpaceInputs,
    },
    types::{ConvergenceTolerance, OdeTolerance},
    ws,
};

use super::{
    state::{Pressure, State, Temperatures},
    Components, Engine,
};

/// Attempt to create a running `Engine`
pub fn run<T: Fluid, U: MatrixDecomposition>(
    components: Components<T>,
    state_hint: State<T>,
    settings: &Settings,
) -> Result<Engine<T>> {
    let mut state = state_hint;
    for _ in 0..settings.max_iters.outer {
        let run = create_run::<T, U>(&components, &state);
        let ic_hint = Conditions {
            P: state.pres.t_zero,
            T_c: 0.5 * (state.temp.sink + state.temp.chx),
            T_e: 0.5 * (state.temp.source + state.temp.hhx),
        };
        let values = run.find_steady_state(
            settings.resolution,
            ic_hint,
            settings.ode_tol,
            settings.loop_tol.inner,
            settings.max_iters.inner,
        )?;
        match state.update(&components, &values, settings.loop_tol.outer) {
            Ok(new_state) => {
                state = new_state;
            }
            Err(old_state) => {
                return Ok(Engine {
                    components,
                    state: old_state,
                    values,
                });
            }
        };
    }

    bail!("not converged")
}

pub struct Settings {
    pub resolution: u32,
    pub loop_tol: LoopTolerance,
    pub ode_tol: OdeTolerance,
    pub max_iters: MaxIters,
}

pub struct LoopTolerance {
    pub inner: ConvergenceTolerance,
    pub outer: ConvergenceTolerance,
}

pub struct MaxIters {
    pub inner: usize,
    pub outer: usize,
}

/// Create an `engine::Run` for a specific matrix solver
#[allow(clippy::similar_names)]
fn create_run<'a, T: Fluid, U: MatrixDecomposition>(
    _components: &'a Components<T>,
    _state: &'a State<T>,
) -> Run<'a, T, U> {
    todo!()
}

/// Information needed to implement `Cycle`
struct Run<'a, T: Fluid, U: MatrixDecomposition> {
    enth_norm: f64,
    fluid: &'a T,
    period: f64,
    pres: Pressure,
    solver: PhantomData<U>,
    temp: Temperatures,
    vol_chx: f64,
    vol_hhx: f64,
    vol_regen: f64,
    ws_parasitics: ws::Parasitics,
    ws_vol_fn: Box<dyn Fn(f64) -> (ws::CompVolume, ws::ExpVolume)>,
}

#[allow(clippy::unused_self)] // TODO: remove when functions are implemented
impl<T: Fluid, U: MatrixDecomposition> Run<'_, T, U> {
    fn comp_inputs(&self, _vol: ws::CompVolume, _temp: f64, _pres: f64) -> WorkingSpaceInputs {
        todo!()
    }

    fn chx_inputs(&self, _pres: f64) -> HeatExchangerInputs {
        todo!()
    }

    fn regen_inputs(&self, _pres: f64) -> RegeneratorInputs {
        todo!()
    }

    fn hhx_inputs(&self, _pres: f64) -> HeatExchangerInputs {
        todo!()
    }

    fn exp_inputs(&self, _vol: ws::ExpVolume, _temp: f64, _pres: f64) -> WorkingSpaceInputs {
        todo!()
    }
}

impl<T: Fluid, U: MatrixDecomposition> Cycle for Run<'_, T, U> {
    type Solver = U;

    fn calculate_inputs(&self, time: f64, conditions: Conditions) -> StateEquationInputs {
        let Conditions { P, T_c, T_e } = conditions;
        let (comp_vol, exp_vol) = (self.ws_vol_fn)(time);
        StateEquationInputs {
            pres: P,
            enth_norm: self.enth_norm,
            comp: self.comp_inputs(comp_vol, T_c, P),
            chx: self.chx_inputs(P),
            regen: self.regen_inputs(P),
            hhx: self.hhx_inputs(P),
            exp: self.exp_inputs(exp_vol, T_e, P),
        }
    }

    fn pres_zero(&self) -> f64 {
        self.pres.t_zero
    }

    fn period(&self) -> f64 {
        self.period
    }
}
