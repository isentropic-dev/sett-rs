use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Inputs {
    pub pres: f64,
    pub enth_norm: f64,
    pub comp: WorkingSpaceInputs,
    pub chx: HeatExchangerInputs,
    pub regen: RegeneratorInputs,
    pub hhx: HeatExchangerInputs,
    pub exp: WorkingSpaceInputs,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct WorkingSpaceInputs {
    pub vol: f64,
    pub dens: f64,
    pub inte: f64,
    pub enth: f64,
    pub dd_dP_T: f64,
    pub dd_dT_P: f64,
    pub du_dP_T: f64,
    pub du_dT_P: f64,
    pub dV_dt: f64,
    pub Q_dot: f64,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct HeatExchangerInputs {
    pub vol: f64,
    pub dens: f64,
    pub inte: f64,
    pub enth: f64,
    pub dd_dP_T: f64,
    pub du_dP_T: f64,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct RegeneratorInputs {
    pub vol: f64,
    pub dens: f64,
    pub inte: f64,
    pub enth_cold: f64,
    pub enth_hot: f64,
    pub dd_dP_T: f64,
    pub du_dP_T: f64,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize)]
pub struct Solution {
    pub m_dot_ck: f64,
    pub m_dot_kr: f64,
    pub m_dot_rl: f64,
    pub m_dot_le: f64,
    pub Q_dot_k: f64,
    pub Q_dot_r: f64,
    pub Q_dot_l: f64,
    pub dTc_dt: f64,
    pub dTe_dt: f64,
    pub dP_dt: f64,
}

/// The elapsed time in seconds since the start of the cycle
pub type Time = f64;

/// The conditions within the engine
///
/// `P`   -- pressure (Pa) in all control volumes
/// `T_c` -- temperature (K) in the compression space
/// `T_e` -- temperature (K) in the expansion space
#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Conditions {
    pub P: f64,
    pub T_c: f64,
    pub T_e: f64,
}

/// The state of the engine at a given time
#[derive(Debug, Clone, Serialize)]
pub struct Values {
    pub time: Time,
    pub conditions: Conditions,
    pub solution: Solution,
}

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Direction {
    Positive,
    Negative,
    Unknown,
}

impl Direction {
    /// Return a `Direction` based on the sign of a number
    ///
    /// If `value` is exactly `0.0`, a positive direction is assumed.
    pub(super) fn from_value(value: f64) -> Self {
        if value >= 0.0 {
            Self::Positive
        } else {
            Self::Negative
        }
    }

    /// Return a value based on the direction of `self`
    ///
    /// An average of the two values is returned if the direction is `Unknown`.
    pub(super) fn select(&self, positive: f64, negative: f64) -> f64 {
        match self {
            Self::Positive => positive,
            Self::Negative => negative,
            Self::Unknown => 0.5 * (positive + negative),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub(super) struct FlowDirection {
    pub(super) ck: Direction,
    pub(super) kr: Direction,
    pub(super) rl: Direction,
    pub(super) le: Direction,
}

impl FlowDirection {
    pub(super) fn from_solution(solution: &Solution) -> Self {
        Self {
            ck: Direction::from_value(solution.m_dot_ck),
            kr: Direction::from_value(solution.m_dot_kr),
            rl: Direction::from_value(solution.m_dot_rl),
            le: Direction::from_value(solution.m_dot_le),
        }
    }
}

impl Default for FlowDirection {
    fn default() -> Self {
        Self {
            ck: Direction::Unknown,
            kr: Direction::Unknown,
            rl: Direction::Unknown,
            le: Direction::Unknown,
        }
    }
}
