mod chx;
mod engine;
mod fluid;
mod hhx;
mod regen;
mod state_equations;
mod ws;

/// Represents parasitic power loss in a component
///
/// Each type of power has units of watts (W).
#[derive(Default)]
pub struct ParasiticPower {
    pub thermal: f64,
    pub mechanical: f64,
    pub electrical: f64,
}
