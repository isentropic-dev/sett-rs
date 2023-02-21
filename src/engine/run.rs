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
    components: Components,
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
            Err(state) => {
                return Ok(Engine {
                    components,
                    state,
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
    _components: &'a Components,
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

impl<T: Fluid, U: MatrixDecomposition> Run<'_, T, U> {
    fn comp_inputs(&self, vol: ws::CompVolume, temp: f64, pres: f64) -> WorkingSpaceInputs {
        WorkingSpaceInputs {
            vol: vol.value,
            dens: self.fluid.dens(temp, pres),
            inte: self.fluid.inte(temp, pres),
            enth: self.fluid.enth(temp, pres),
            dd_dP_T: self.fluid.dd_dP_T(temp, pres),
            dd_dT_P: self.fluid.dd_dT_P(temp, pres),
            du_dP_T: self.fluid.du_dP_T(temp, pres),
            du_dT_P: self.fluid.du_dT_P(temp, pres),
            dV_dt: vol.deriv,
            Q_dot: self.ws_parasitics.comp.thermal,
        }
    }

    fn chx_inputs(&self, pres: f64) -> HeatExchangerInputs {
        let temp = self.temp.chx;
        HeatExchangerInputs {
            vol: self.vol_chx,
            dens: self.fluid.dens(temp, pres),
            inte: self.fluid.inte(temp, pres),
            enth: self.fluid.enth(temp, pres),
            dd_dP_T: self.fluid.dd_dP_T(temp, pres),
            du_dP_T: self.fluid.du_dP_T(temp, pres),
        }
    }

    fn regen_inputs(&self, pres: f64) -> RegeneratorInputs {
        let temp = self.temp.regen_avg;
        RegeneratorInputs {
            vol: self.vol_regen,
            dens: self.fluid.dens(temp, pres),
            inte: self.fluid.inte(temp, pres),
            dd_dP_T: self.fluid.dd_dP_T(temp, pres),
            du_dP_T: self.fluid.du_dP_T(temp, pres),
            enth_cold: self.fluid.enth(self.temp.regen_cold, pres),
            enth_hot: self.fluid.enth(self.temp.regen_hot, pres),
        }
    }

    fn hhx_inputs(&self, pres: f64) -> HeatExchangerInputs {
        let temp = self.temp.hhx;
        HeatExchangerInputs {
            vol: self.vol_hhx,
            dens: self.fluid.dens(temp, pres),
            inte: self.fluid.inte(temp, pres),
            enth: self.fluid.enth(temp, pres),
            dd_dP_T: self.fluid.dd_dP_T(temp, pres),
            du_dP_T: self.fluid.du_dP_T(temp, pres),
        }
    }

    fn exp_inputs(&self, vol: ws::ExpVolume, temp: f64, pres: f64) -> WorkingSpaceInputs {
        WorkingSpaceInputs {
            vol: vol.value,
            dens: self.fluid.dens(temp, pres),
            inte: self.fluid.inte(temp, pres),
            enth: self.fluid.enth(temp, pres),
            dd_dP_T: self.fluid.dd_dP_T(temp, pres),
            dd_dT_P: self.fluid.dd_dT_P(temp, pres),
            du_dP_T: self.fluid.du_dP_T(temp, pres),
            du_dT_P: self.fluid.du_dT_P(temp, pres),
            dV_dt: vol.deriv,
            Q_dot: self.ws_parasitics.exp.thermal,
        }
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
