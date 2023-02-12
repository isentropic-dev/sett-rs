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
