use serde::Deserialize;

pub const DEFAULT_MAX_ITERS: u32 = 20;

/// An error that can occur during an engine run
/// TODO: <https://github.com/isentropic-dev/sett-rs/issues/64>
/// TODO: <https://github.com/isentropic-dev/sett-rs/issues/65>
#[derive(Debug, Clone, Copy)]
pub enum RunError {
    InnerLoop,
    OuterLoop,
}

/// Inputs to an engine run
#[derive(Debug, Clone, Copy)]
pub struct RunInputs {
    pub pres_zero: f64,
    pub temp_sink: f64,
    pub temp_source: f64,
}

/// Settings for an engine run
#[derive(Debug, Clone, Copy)]
pub struct RunSettings {
    pub resolution: u32,
    pub loop_tol: LoopTolerance,
    pub ode_tol: OdeTolerance,
    pub max_iters: MaxIters,
}

/// Tolerances related to the two iteration loops
#[derive(Debug, Clone, Copy)]
pub struct LoopTolerance {
    pub inner: ConvergenceTolerance,
    pub outer: ConvergenceTolerance,
}

/// Tolerances used by the ODE integrator
#[derive(Debug, Clone, Copy)]
pub struct OdeTolerance {
    pub abs: f64,
    pub rel: f64,
}

/// Tolerances related to convergence between subsequent values
#[derive(Debug, Clone, Copy)]
pub struct ConvergenceTolerance {
    pub abs: f64,
    pub rel: f64,
}

/// Number of iterations to try before failing
#[derive(Debug, Clone, Copy)]
pub struct MaxIters {
    pub inner: usize,
    pub outer: usize,
}

/// Parasitic power loss in a component
///
/// Each type of power has units of watts (W).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ParasiticPower {
    pub thermal: f64,
    pub mechanical: f64,
    pub electrical: f64,
}

/// Average conditions in a heat exchanger
#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy)]
pub struct HeatExchanger {
    pub temp: f64,
    pub pres: f64,
    pub dens: f64,
    pub cp: f64,
    pub m_dot: f64,
    pub Q_dot: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SolverConfig {
    pub inner_loop: InnerLoopConfig,
    pub outer_loop: OuterLoopConfig,
    pub ode: OdeConfig,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LegacySolverConfig {
    pub inner_loop_tolerance: ToleranceConfig,
    pub outer_loop_tolerance: ToleranceConfig,
    pub ode_tolerance: ToleranceConfig,
    pub ode_solver: LegacyOdeSolver,
    pub time_resolution: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum LegacyOdeSolver {
    Ode45,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct InnerLoopConfig {
    pub tolerance: ToleranceConfig,
    pub max_iterations: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct OuterLoopConfig {
    pub tolerance: ToleranceConfig,
    pub max_iterations: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct OdeConfig {
    pub tolerance: ToleranceConfig,
    pub num_timesteps: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ToleranceConfig {
    pub abs: f64,
    pub rel: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ConditionsConfig {
    pub temp_sink: f64,
    pub temp_source: f64,
    pub pres_zero: f64,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct LegacyConditionsConfig {
    pub T_cold: f64,
    pub T_hot: f64,
    pub P_0: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Material {
    SS304,
    Multimet,
    StainlessSteel,
}

impl OdeTolerance {
    #[must_use]
    pub fn new(abs: f64, rel: f64) -> Self {
        Self { abs, rel }
    }
}

impl ConvergenceTolerance {
    #[must_use]
    pub fn new(abs: f64, rel: f64) -> Self {
        Self { abs, rel }
    }

    /// Return `true` if the change from `old` to `new` is sufficiently small
    #[must_use]
    pub fn is_converged(&self, old: f64, new: f64) -> bool {
        let abs_change = new - old;
        let rel_change = abs_change / old;
        abs_change.abs() < self.abs && rel_change.abs() < self.rel
    }
}

impl From<ToleranceConfig> for ConvergenceTolerance {
    fn from(config: ToleranceConfig) -> Self {
        Self {
            abs: config.abs,
            rel: config.rel,
        }
    }
}

impl From<SolverConfig> for RunSettings {
    fn from(config: SolverConfig) -> Self {
        Self {
            resolution: config.ode.num_timesteps,
            loop_tol: LoopTolerance {
                inner: config.inner_loop.tolerance.into(),
                outer: config.outer_loop.tolerance.into(),
            },
            ode_tol: OdeTolerance {
                abs: config.ode.tolerance.abs,
                rel: config.ode.tolerance.rel,
            },
            max_iters: MaxIters {
                inner: config.inner_loop.max_iterations as usize,
                outer: config.outer_loop.max_iterations as usize,
            },
        }
    }
}

impl From<LegacySolverConfig> for SolverConfig {
    fn from(config: LegacySolverConfig) -> Self {
        Self {
            inner_loop: InnerLoopConfig {
                tolerance: config.inner_loop_tolerance,
                max_iterations: DEFAULT_MAX_ITERS,
            },
            outer_loop: OuterLoopConfig {
                tolerance: config.outer_loop_tolerance,
                max_iterations: DEFAULT_MAX_ITERS,
            },
            ode: OdeConfig {
                tolerance: config.ode_tolerance,
                num_timesteps: config.time_resolution,
            },
        }
    }
}

impl From<ConditionsConfig> for RunInputs {
    fn from(config: ConditionsConfig) -> Self {
        Self {
            pres_zero: config.pres_zero,
            temp_sink: config.temp_sink,
            temp_source: config.temp_source,
        }
    }
}

impl From<LegacyConditionsConfig> for ConditionsConfig {
    fn from(config: LegacyConditionsConfig) -> Self {
        Self {
            temp_sink: config.T_cold,
            temp_source: config.T_hot,
            pres_zero: config.P_0,
        }
    }
}
