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

impl OdeTolerance {
    pub fn new(abs: f64, rel: f64) -> Self {
        Self { abs, rel }
    }
}

impl ConvergenceTolerance {
    pub fn new(abs: f64, rel: f64) -> Self {
        Self { abs, rel }
    }

    /// Return `true` if the change from `old` to `new` is sufficiently small
    pub fn is_converged(&self, old: f64, new: f64) -> bool {
        let abs_change = new - old;
        let rel_change = abs_change / old;
        abs_change.abs() < self.abs && rel_change.abs() < self.rel
    }
}
