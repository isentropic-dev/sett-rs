mod config;
mod engine;
mod performance;
mod state_equations;

pub mod api;
pub mod chx;
pub mod fluid;
pub mod hhx;
pub mod regen;
pub mod types;
pub mod ws;

use crate::api::RunResults;
pub use crate::config::{Config, Legacy};
pub use engine::{Components, Engine};
pub use state_equations::{LuSolver, QrSolver, SvdDefaultSolver};

pub use api::run_engine;

/// An example of how to run an engine from a config
///
/// This function will be removed when the MATLAB interface is implemented.
/// See: <https://github.com/isentropic-dev/sett-rs/issues/40>
///
/// # Panics
///
/// If an unsupported fluid model is provided.
pub fn run_from_config(config: impl Into<Config>) {
    let config = config.into();
    let fluid = match config.engine.fluid {
        fluid::Config::Hydrogen(model) => match model {
            fluid::ModelConfig::IdealGas => fluid::IdealGas::hydrogen(),
            fluid::ModelConfig::RefProp => todo!(),
            fluid::ModelConfig::Fit => todo!(),
            fluid::ModelConfig::Custom => todo!(),
        },
        fluid::Config::Helium(model) => match model {
            fluid::ModelConfig::IdealGas => fluid::IdealGas::helium(),
            fluid::ModelConfig::RefProp => todo!(),
            fluid::ModelConfig::Fit => todo!(),
            fluid::ModelConfig::Custom => todo!(),
        },
    };

    let engine = Engine::run::<LuSolver>(
        config.engine.components.into(),
        fluid,
        config.conditions.into(),
        config.solver.into(),
    )
    .expect("engine should converge");

    println!("-----------------");
    println!("time:  {:?}", engine.values.time);
    println!("P:     {:?}", engine.values.P);
    println!("T_c:   {:?}", engine.values.T_c);
    println!("T_e:   {:?}", engine.values.T_e);

    let results = RunResults::from(engine);

    println!("Results: {results:?}");
}
