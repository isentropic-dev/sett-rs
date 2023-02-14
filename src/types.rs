/// Represents the environment an engine is run in
#[derive(Debug, Clone, Copy)]
pub struct Environment {
    /// The lowest temperature (K) available to the engine
    pub sink_temp: f64,

    /// The highest temperature (K) available to the engine
    pub source_temp: f64,
}

/// Represents parasitic power loss in a component
///
/// Each type of power has units of watts (W).
#[derive(Debug, Clone, Copy, Default)]
pub struct ParasiticPower {
    pub thermal: f64,
    pub mechanical: f64,
    pub electrical: f64,
}

/// Tolerances used by the ODE integrator
#[derive(Debug, Clone, Copy)]
pub struct OdeTolerance {
    pub abs: f64,
    pub rel: f64,
}

impl OdeTolerance {
    pub fn new(abs: f64, rel: f64) -> Self {
        Self { abs, rel }
    }
}

/// Tolerances related to convergence between subsequent values
#[derive(Debug, Clone, Copy)]
pub struct ConvergenceTolerance {
    pub abs: f64,
    pub rel: f64,
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
