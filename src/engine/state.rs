use crate::{fluid::Fluid, state_equations::Values, types::ConvergenceTolerance};

use super::Components;

/// The thermodynamic state of a running Stirling engine
pub struct State<T: Fluid> {
    pub fluid: T,
    pub temp: Temperatures,
    pub pres: Pressure,
    pub mass_flow: MassFlows,
    pub heat_flow: HeatFlows,
    pub regen_imbalance: RegenImbalance,
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
        _values: &[Values],
        _tol: ConvergenceTolerance,
    ) -> Result<Self, Self> {
        todo!()
    }
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

/// Engine pressure over the cycle in Pa
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pressure {
    pub avg: f64,
    pub max: f64,
    pub min: f64,
    pub t_zero: f64,
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

/// Average mass flow rates through the heat exchangers in kg/s
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MassFlows {
    pub chx: f64,
    pub regen: f64,
    pub hhx: f64,
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

/// Average heat flow rates through the heat exhangers in W
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeatFlows {
    pub chx: f64,
    pub regen: f64,
    pub hhx: f64,
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

#[cfg(test)]
mod tests {
    use crate::fluid::IdealGas;

    use super::*;

    #[test]
    fn can_create_state() {
        let fluid = IdealGas::new("hydrogen").unwrap();
        let _state = State {
            fluid,
            temp: Temperatures {
                sink: 300.0,
                chx: 310.0,
                regen_cold: 400.0,
                regen_avg: 450.0,
                regen_hot: 500.0,
                hhx: 590.0,
                source: 600.0,
            },
            pres: Pressure::constant(10e6),
            mass_flow: MassFlows::constant(1.0),
            heat_flow: HeatFlows::constant(1.0),
            regen_imbalance: RegenImbalance::default(),
        };
    }
}
