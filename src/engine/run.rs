use std::marker::PhantomData;

use crate::{
    fluid::Fluid,
    state_equations::{
        Conditions, Cycle, HeatExchangerInputs, Inputs as StateEquationInputs, MatrixDecomposition,
        RegeneratorInputs, WorkingSpaceInputs,
    },
    ws,
};

use super::{
    state::{Pressure, State, Temperatures},
    Components,
};

/// Information needed to implement `Cycle`
pub(super) struct Run<'a, T: Fluid, U: MatrixDecomposition> {
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

impl<'a, T: Fluid, U: MatrixDecomposition> Run<'a, T, U> {
    /// Create an `Run` for a specific matrix solver
    #[allow(clippy::similar_names)]
    pub(super) fn new(components: &'a Components, state: &'a State<T>) -> Self {
        // Calculate an average enthalpy using the sink and source temperatures
        let h_sink = state.fluid.enth(state.temp.sink, state.pres.avg);
        let h_source = state.fluid.enth(state.temp.source, state.pres.avg);
        let enth_norm = 0.5 * (h_sink + h_source);

        // Ask heat exchanger components for their volumes
        let vol_chx = components.chx.volume();
        let vol_regen = components.regen.volume();
        let vol_hhx = components.hhx.volume();

        // Ask working spaces component for its properties
        let ws_state = state.ws();
        let period = 1.0 / components.ws.frequency(&ws_state);
        let ws_vol_fn = components.ws.volumes(&ws_state);
        let ws_parasitics = components.ws.parasitics(&ws_state);

        Self {
            enth_norm,
            fluid: &state.fluid,
            period,
            pres: state.pres,
            solver: PhantomData,
            temp: state.temp,
            vol_chx,
            vol_hhx,
            vol_regen,
            ws_parasitics,
            ws_vol_fn,
        }
    }

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
        let temp = self.temp.regen.avg;
        RegeneratorInputs {
            vol: self.vol_regen,
            dens: self.fluid.dens(temp, pres),
            inte: self.fluid.inte(temp, pres),
            dd_dP_T: self.fluid.dd_dP_T(temp, pres),
            du_dP_T: self.fluid.du_dP_T(temp, pres),
            enth_cold: self.fluid.enth(self.temp.regen.cold, pres),
            enth_hot: self.fluid.enth(self.temp.regen.hot, pres),
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
