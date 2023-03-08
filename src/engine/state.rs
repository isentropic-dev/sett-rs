use itertools::Itertools;

use crate::{
    chx,
    fluid::Fluid,
    hhx, regen, state_equations,
    types::{ConvergenceTolerance, HeatExchanger, RunInputs},
    ws,
};

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
    pub sink: f64,
    pub chx: f64,
    pub regen: RegenTemp,
    pub hhx: f64,
    pub source: f64,
}

/// Temperatures associated with the regenerator
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegenTemp {
    pub cold: f64, // T_r_cold
    pub avg: f64,  // T_r
    pub hot: f64,  // T_r_hot
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
///   `T_r_hot = T_l - (approach + imbalance)`
///
/// If a `RegenImbalance` is negative, the cold and hot sides are calculated according to:
///
///    `T_r_cold = T_k + (approach - imbalance)`
///    `T_r_hot = T_l - approach`
///
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct RegenImbalance(f64);

/// Time-discretized state values within a Stirling engine
#[allow(non_snake_case)]
#[derive(Debug, Default)]
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
}

impl<T: Fluid> State<T> {
    /// Return `self` updated from new `state_equations::Values`
    ///
    /// The updated `State` is returned as `Ok(self)`.  If the provided
    /// `values` do not change the `State` within `tol`, then the original
    /// `State` is returned as `Err(self)`.
    #[allow(clippy::result_large_err)]
    pub fn update(
        self,
        _components: &Components,
        _values: &Values,
        _tol: ConvergenceTolerance,
    ) -> Result<Self, Self> {
        Err(self) // TODO: for now we assume state is always converged
    }

    /// Return the `ws::State` that corresponds to this `engine::State`
    pub(super) fn ws(&self) -> ws::State {
        ws::State { pres: self.pres }
    }

    /// Return the `chx::State` that corresponds to this `engine::State`
    pub(super) fn chx(&self) -> chx::State {
        chx::State {
            hxr: HeatExchanger {
                temp: self.temp.chx,
                pres: self.pres.avg,
                dens: self.fluid.dens(self.temp.chx, self.pres.avg),
                cp: self.fluid.cp(self.temp.chx, self.pres.avg),
                m_dot: self.mass_flow.chx,
                Q_dot: self.heat_flow.chx,
            },
            sink_temp: self.temp.sink,
        }
    }

    /// Return the `regen::State` that corresponds to this `engine::State`
    pub(super) fn regen(&self) -> regen::State {
        regen::State {
            hxr: HeatExchanger {
                temp: self.temp.regen.avg,
                pres: self.pres.avg,
                dens: self.fluid.dens(self.temp.regen.avg, self.pres.avg),
                cp: self.fluid.cp(self.temp.regen.avg, self.pres.avg),
                m_dot: self.mass_flow.regen,
                Q_dot: self.heat_flow.regen,
            },
        }
    }

    /// Return the `hhx::State` that corresponds to this `engine::State`
    pub(super) fn hhx(&self) -> hhx::State {
        hhx::State {
            hxr: HeatExchanger {
                temp: self.temp.hhx,
                pres: self.pres.avg,
                dens: self.fluid.dens(self.temp.hhx, self.pres.avg),
                cp: self.fluid.cp(self.temp.hhx, self.pres.avg),
                m_dot: self.mass_flow.hhx,
                Q_dot: self.heat_flow.hhx,
            },
            source_temp: self.temp.source,
        }
    }

    /// Create an initial `State` hint
    #[allow(clippy::similar_names)]
    pub(super) fn new_hint(components: &Components, fluid: T, inputs: RunInputs) -> Self {
        let RunInputs {
            pres_zero,
            temp_sink,
            temp_source,
        } = inputs;

        // Make some initial state in order to get approach temperatures from components
        let regen_imbalance = RegenImbalance::default();
        let mut state = Self {
            fluid,
            pres: Pressure::constant(pres_zero),
            temp: Temperatures::from_env(temp_sink, temp_source),
            mass_flow: MassFlows::constant(0.),
            heat_flow: HeatFlows::constant(0.),
            regen_imbalance,
        };

        // Request approach temperatures
        let chx_approach = components.chx.initial_approach();
        let regen_approach = components.regen.initial_approach();
        let hhx_approach = components.hhx.initial_approach();

        // Use approaches to update temperatures in the initial state
        let temp_chx = temp_sink + chx_approach;
        let temp_hhx = temp_source - hhx_approach;
        let temp_regen = regen_imbalance.regen_temp(temp_chx, temp_hhx, regen_approach);

        state.temp.chx = temp_chx;
        state.temp.regen = temp_regen;
        state.temp.hhx = temp_hhx;

        state
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

    /// Calculate `Pressure` from `Values`
    pub fn from_values(values: &Values) -> Self {
        let t_final = values.final_time();
        let avg = integrate(&values.time, &values.P) / t_final;
        let t_zero = values.P[0];
        let max = *values
            .P
            .iter()
            .max_by(|a, b| a.total_cmp(b))
            .expect("values cannot be empty");
        let min = *values
            .P
            .iter()
            .min_by(|a, b| a.total_cmp(b))
            .expect("values cannot be empty");

        Self {
            avg,
            max,
            min,
            t_zero,
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
        let regen = RegenTemp {
            cold: (chx + regen_avg) * 0.5,
            avg: regen_avg,
            hot: (hhx + regen_avg) * 0.5,
        };
        Self {
            sink,
            chx,
            regen,
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

    /// Calculate `MassFlows` from `Values`
    pub fn from_values(values: &Values) -> Self {
        Self {
            chx: values.m_dot_chx(),
            regen: values.m_dot_regen(),
            hhx: values.m_dot_hhx(),
        }
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

    /// Calculate `HeatFlows` from `Values`
    ///
    /// To calculate the total heat flow in the cold and hot heat exchangers
    /// we must also account for heat flow from the fluid in the compression
    /// and expansion spaces to their respective heat exchangers.  Calculating
    /// this additional heat transfer requires us to provide the heat exchanger
    /// temperatures and the thermal resistance of the fluid in the two spaces.
    #[allow(non_snake_case, clippy::similar_names)]
    pub fn from_values(
        values: &Values,
        temp_chx: f64,
        temp_hhx: f64,
        thermal_res: ws::ThermalResistance,
    ) -> Self {
        let t_final = values.final_time();

        // Cold heat exchanger
        let Q_dot_c = if thermal_res.comp.is_infinite() {
            0. // no heat flow if compression space is adiabatic
        } else {
            let Q_dot_c: Vec<_> = values
                .T_c
                .iter()
                .map(|T_c| (T_c - temp_chx) / thermal_res.comp) // heat flow from fluid in compression space to chx
                .collect();
            integrate(&values.time, &Q_dot_c) / t_final
        };
        let Q_dot_k = integrate(&values.time, &values.Q_dot_k) / t_final;
        let chx = Q_dot_c + Q_dot_k;

        // Regenerator
        let regen = integrate(&values.time, &values.Q_dot_r) / t_final;

        // Hot heat exchanger
        let Q_dot_e = if thermal_res.exp.is_infinite() {
            0. // no heat flow if expansion space is adiabatic
        } else {
            let Q_dot_e: Vec<_> = values
                .T_e
                .iter()
                .map(|T_e| (temp_hhx - T_e) / thermal_res.exp) // heat flow from hhx to fluid in expansion space
                .collect();
            integrate(&values.time, &Q_dot_e) / t_final
        };
        let Q_dot_l = integrate(&values.time, &values.Q_dot_l) / t_final;
        let hhx = Q_dot_e + Q_dot_l;

        Self { chx, regen, hhx }
    }
}

impl Values {
    /// Return the last `self.time` value
    fn final_time(&self) -> f64 {
        *self.time.last().expect("values cannot be empty")
    }

    /// Calculate the average mass flow rate through the cold heat exchanger
    fn m_dot_chx(&self) -> f64 {
        self.m_dot_avg(self.m_dot_ck.iter(), self.m_dot_kr.iter())
    }

    /// Calculate the average mass flow rate through the regenerator
    fn m_dot_regen(&self) -> f64 {
        self.m_dot_avg(self.m_dot_kr.iter(), self.m_dot_rl.iter())
    }

    /// Calculate the average mass flow rate through the hot heat exchanger
    fn m_dot_hhx(&self) -> f64 {
        self.m_dot_avg(self.m_dot_rl.iter(), self.m_dot_le.iter())
    }

    /// Calculate the average mass flow rate through a control volume
    ///
    /// The control volume is defined by the two iterators that provide time-
    /// discretized mass flow rates across the `left` and `right` sides of it.
    fn m_dot_avg<'a>(
        &self,
        left: impl Iterator<Item = &'a f64>,
        right: impl Iterator<Item = &'a f64>,
    ) -> f64 {
        let m_dot: Vec<_> = left
            .zip(right)
            .map(|(l, r)| 0.5 * (l + r).abs()) // flow in different directions cancel each other before taking abs
            .collect();
        integrate(&self.time, &m_dot) / self.final_time()
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

        // Fill vectors using a single iteration over values
        for value in values {
            time.push(value.time);
            P.push(value.conditions.P);
            T_c.push(value.conditions.T_c);
            T_e.push(value.conditions.T_e);
            m_dot_ck.push(value.solution.m_dot_ck);
            m_dot_kr.push(value.solution.m_dot_kr);
            m_dot_rl.push(value.solution.m_dot_rl);
            m_dot_le.push(value.solution.m_dot_le);
            Q_dot_k.push(value.solution.Q_dot_k);
            Q_dot_r.push(value.solution.Q_dot_r);
            Q_dot_l.push(value.solution.Q_dot_l);
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
        }
    }
}

impl RegenImbalance {
    /// Calculate a `RegenTemperature`
    #[allow(clippy::similar_names)]
    pub fn regen_temp(self, temp_chx: f64, temp_hhx: f64, approach: f64) -> RegenTemp {
        let cold_approach = if self.0 >= 0. {
            approach
        } else {
            approach - self.0
        };
        let hot_approach = if self.0 >= 0. {
            approach + self.0
        } else {
            approach
        };

        let cold = temp_chx + cold_approach;
        let hot = temp_hhx - hot_approach;
        let avg = (temp_hhx - 0.5 * hot_approach - temp_chx - 0.5 * cold_approach)
            / ((temp_hhx - 0.5 * hot_approach) / (temp_chx + 0.5 * cold_approach)).ln();

        RegenTemp { cold, avg, hot }
    }
}

/// Integrate `y` over `x` using the trapezoidal rule
fn integrate(x: &[f64], y: &[f64]) -> f64 {
    let xs = x.iter().tuple_windows();
    let ys = y.iter().tuple_windows();
    xs.zip(ys)
        .map(|((x0, x1), (y0, y1))| (y1 + y0) * (x1 - x0) * 0.5)
        .sum()
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

    #[test]
    fn pressure_from_values() {
        // Constant pressure
        let values = Values {
            time: vec![0.0, 10.0],
            P: vec![1.0, 1.0],
            ..Values::default()
        };
        let expected = Pressure::constant(1.0);
        assert_eq!(expected, Pressure::from_values(&values));

        // Simple integration
        let values = Values {
            time: vec![0.0, 1.0, 2.0],
            P: vec![100.0, 200.0, 100.0],
            ..Values::default()
        };
        let expected = Pressure {
            avg: 150.0,
            max: 200.0,
            min: 100.0,
            t_zero: 100.0,
        };
        assert_eq!(expected, Pressure::from_values(&values));

        // Integration with more points
        let values = Values {
            time: vec![0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0],
            P: vec![1.2e3, 2.5e3, 1.9e2, 1.7e2, 3.2e3, 8.8e3, 6.5e3, 1.4e3, 2e2],
            ..Values::default()
        };
        let expected = Pressure {
            avg: 2932.5,
            max: 8800.0,
            min: 170.0,
            t_zero: 1200.0,
        };
        assert_eq!(expected, Pressure::from_values(&values));
    }

    #[test]
    fn mass_flows_from_values() {
        // Constant mass flow rates
        let values = Values {
            time: vec![0.0, 5.0],
            m_dot_ck: vec![1.0, 1.0],
            m_dot_kr: vec![2.0, 2.0],
            m_dot_rl: vec![3.0, 3.0],
            m_dot_le: vec![4.0, 4.0],
            ..Values::default()
        };
        let expected = MassFlows {
            chx: 1.5,
            regen: 2.5,
            hhx: 3.5,
        };
        assert_eq!(expected, MassFlows::from_values(&values));

        // Check that flow directions cancel
        let values = Values {
            time: vec![0.0, 5.0, 10.0],
            m_dot_ck: vec![1.0, 1.0, 1.0],
            m_dot_kr: vec![-2.0, -2.0, -2.0],
            m_dot_rl: vec![-3.0, -3.0, -3.0],
            m_dot_le: vec![4.0, -4.0, -3.0],
            ..Values::default()
        };
        let expected = MassFlows {
            chx: 0.5,
            regen: 2.5,
            hhx: 2.625,
        };
        assert_eq!(expected, MassFlows::from_values(&values));
    }

    #[test]
    fn heat_flows_from_values() {
        // Simple heat flow rates with adiabatic working spaces
        let values = Values {
            time: vec![0.0, 50.0, 100.0],
            Q_dot_k: vec![1.0, 1.0, 1.0],
            Q_dot_r: vec![1.0, 0.0, -1.0],
            Q_dot_l: vec![0.0, 20.0, 0.0],
            ..Values::default()
        };
        let temp_chx = 300.0;
        let temp_hhx = 800.0;
        let thermal_res = ws::ThermalResistance::default(); // default is `f64::INFINITY`
        let expected = HeatFlows {
            chx: 1.0,
            regen: 0.0,
            hhx: 10.0,
        };
        let actual = HeatFlows::from_values(&values, temp_chx, temp_hhx, thermal_res);
        assert_eq!(expected, actual);

        // Check non-adiabatic conditions
        let values = Values {
            time: vec![0.0, 1.0],
            T_c: vec![400.0, 400.0],
            T_e: vec![600.0, 600.0],
            Q_dot_k: vec![1.0, 1.0],
            Q_dot_l: vec![5.0, 5.0],
            ..Values::default()
        };
        let temp_chx = 300.0;
        let temp_hhx = 800.0;
        let thermal_res = ws::ThermalResistance {
            comp: 1.0,
            exp: 50.0,
        };
        let expected = HeatFlows {
            chx: 101.0,
            regen: 0.0,
            hhx: 9.0,
        };
        let actual = HeatFlows::from_values(&values, temp_chx, temp_hhx, thermal_res);
        assert_eq!(expected, actual);
    }
}
