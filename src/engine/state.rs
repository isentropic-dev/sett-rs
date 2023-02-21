use crate::{fluid::Fluid, state_equations, types::ConvergenceTolerance};

use super::Components;

/// The state of a running Stirling engine
pub struct State<T: Fluid> {
    pub fluid: T,
    pub pres: Pressure,
    pub temp: Temperatures,
    pub mass_flow: MassFlows,
    pub heat_flow: HeatFlows,
    pub regen_imbalance: RegenImbalance,
}

/// Engine pressure over the cycle in Pa
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pressure {
    pub avg: f64,
    pub max: f64,
    pub min: f64,
    pub t_zero: f64,
}

/// Constant engine temperatures in K
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Temperatures {
    pub sink: f64,       // T_cold
    pub chx: f64,        // T_k
    pub regen_cold: f64, // T_r_cold
    pub regen_avg: f64,  // T_r
    pub regen_hot: f64,  // T_r_hot
    pub hhx: f64,        // T_l
    pub source: f64,     // T_hot
}

/// Average mass flow rates through the heat exchangers in kg/s
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MassFlows {
    pub chx: f64,
    pub regen: f64,
    pub hhx: f64,
}

/// Average heat flow rates through the heat exhangers in W
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeatFlows {
    pub chx: f64,
    pub regen: f64,
    pub hhx: f64,
}

/// The regenerator approach temperature imbalance in K
///
/// A regenerator is a unique heat exhanger in that it effectively has two
/// approach temperatures; a cold side approach and a hot side approach.
///
/// When using a single approach temperature to characterize a regnerator's
/// performance, that approach is assumed to be the smaller of these two
/// temperature differences.  Given a single approach temperature, the
/// `RegenImbalance` is used to calculate the other one.
///
/// If a `RegenImbalance` is positive, the cold and hot sides are calculated according to:
///
///   `T_r_cold = T_k + approach`
///   `T_r_hot = T_l - approach - imbalance`
///
/// If a `RegenImbalance` is negative, the cold and hot sides are calculated according to:
///
///    `T_r_cold = T_k + approach - imbalance`
///    `T_r_hot = T_l - approach`
///
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct RegenImbalance(f64);

/// Time-discretized state values within a Stirling engine
#[allow(non_snake_case)]
#[derive(Default)]
pub struct Values {
    pub time: Vec<f64>,
    pub P: Vec<f64>,
    pub T_c: Vec<f64>,
    pub T_e: Vec<f64>,
    pub m_dot_ck: Vec<f64>,
    pub m_dot_kr: Vec<f64>,
    pub m_dot_rl: Vec<f64>,
    pub m_dot_le: Vec<f64>,
    pub Q_dot_k: Vec<f64>,
    pub Q_dot_r: Vec<f64>,
    pub Q_dot_l: Vec<f64>,
    pub dP_dt: Vec<f64>,
}

/// Average conditions in a heat exchanger
#[allow(non_snake_case)]
pub struct HeatExchanger {
    pub temp: f64,
    pub pres: f64,
    pub dens: f64,
    pub cp: f64,
    pub m_dot: f64,
    pub Q_dot: f64,
}

impl<T: Fluid> State<T> {
    /// Return `self` updated from new `state_equations::Values`
    ///
    /// The updated `State` is returned as `Ok(self)`.  If the provided
    /// `values` do not change the `State` within `tol`, then the original
    /// `State` is returned as `Err(self)`.
    #[allow(clippy::result_large_err)]
    #[allow(clippy::unused_self)] // TODO: remove when function is implemented
    pub fn update(
        self,
        _components: &Components,
        _values: &Values,
        _tol: ConvergenceTolerance,
    ) -> Result<Self, Self> {
        todo!()
    }
}

impl Pressure {
    /// Create a `Pressure` that is constant over a cycle
    pub fn constant(value: f64) -> Self {
        Self {
            avg: value,
            max: value,
            min: value,
            t_zero: value,
        }
    }
}

impl Temperatures {
    /// Construct `Temperatures` from sink (`T_cold`) and source (`T_hot`) temperatures
    #[allow(clippy::similar_names)]
    pub fn from_env(sink: f64, source: f64) -> Self {
        let chx = sink;
        let hhx = source;
        let regen_avg = (sink + source) * 0.5;
        let regen_cold = (chx + regen_avg) * 0.5;
        let regen_hot = (hhx + regen_avg) * 0.5;
        Self {
            sink,
            chx,
            regen_cold,
            regen_avg,
            regen_hot,
            hhx,
            source,
        }
    }
}

impl MassFlows {
    /// Assume all `MassFlows` are a constant `value`
    pub fn constant(value: f64) -> Self {
        Self {
            chx: value,
            regen: value,
            hhx: value,
        }
    }

    /// Calculate `MassFlows` from `state_equations::Values`
    pub fn from_state_values(_values: &[Values]) -> Self {
        todo!()
    }
}

impl HeatFlows {
    /// Assume all `HeatFlows` are a constant `value`
    pub fn constant(value: f64) -> Self {
        Self {
            chx: value,
            regen: value,
            hhx: value,
        }
    }

    /// Calculate `HeatFlows` from `state_equations::Values`
    pub fn from_state_values(_values: &[Values], _temp_chx: f64, _thermal_res_comp: f64) -> Self {
        todo!()
    }
}

impl From<Vec<state_equations::Values>> for Values {
    #[allow(non_snake_case)]
    fn from(values: Vec<state_equations::Values>) -> Self {
        // Initialize all vectors with their known capacity
        let size = values.len();
        let mut time = Vec::with_capacity(size);
        let mut P = Vec::with_capacity(size);
        let mut T_c = Vec::with_capacity(size);
        let mut T_e = Vec::with_capacity(size);
        let mut m_dot_ck = Vec::with_capacity(size);
        let mut m_dot_kr = Vec::with_capacity(size);
        let mut m_dot_rl = Vec::with_capacity(size);
        let mut m_dot_le = Vec::with_capacity(size);
        let mut Q_dot_k = Vec::with_capacity(size);
        let mut Q_dot_r = Vec::with_capacity(size);
        let mut Q_dot_l = Vec::with_capacity(size);
        let mut dP_dt = Vec::with_capacity(size);

        // Fill vectors using a single iteration over values
        for (i, v) in values.into_iter().enumerate() {
            time[i] = v.time;
            P[i] = v.conditions.P;
            T_c[i] = v.conditions.T_c;
            T_e[i] = v.conditions.T_e;
            m_dot_ck[i] = v.solution.m_dot_ck;
            m_dot_kr[i] = v.solution.m_dot_kr;
            m_dot_rl[i] = v.solution.m_dot_rl;
            m_dot_le[i] = v.solution.m_dot_le;
            Q_dot_k[i] = v.solution.Q_dot_k;
            Q_dot_r[i] = v.solution.Q_dot_r;
            Q_dot_l[i] = v.solution.Q_dot_l;
            dP_dt[i] = v.solution.dP_dt;
        }

        Self {
            time,
            P,
            T_c,
            T_e,
            m_dot_ck,
            m_dot_kr,
            m_dot_rl,
            m_dot_le,
            Q_dot_k,
            Q_dot_r,
            Q_dot_l,
            dP_dt,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fluid::IdealGas;

    use super::*;

    #[test]
    fn can_create_state() {
        let fluid = IdealGas::hydrogen();
        let _state = State {
            fluid,
            temp: Temperatures::from_env(300.0, 600.0),
            pres: Pressure::constant(10e6),
            mass_flow: MassFlows::constant(1.0),
            heat_flow: HeatFlows::constant(1.0),
            regen_imbalance: RegenImbalance::default(),
        };
    }
}
