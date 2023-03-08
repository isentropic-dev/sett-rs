mod config;
mod engine;
mod state_equations;

pub mod api;
pub mod chx;
pub mod fluid;
pub mod hhx;
pub mod regen;
pub mod types;
pub mod ws;

pub use crate::config::Config;
pub use engine::{Components, Engine};
pub use state_equations::{LuSolver, QrSolver, SvdDefaultSolver};

/// # Panics
///
/// If an unsupported fluid model is provided.
pub fn run_from_config(config: Config) {
    let fluid = match config.engine.fluid {
        fluid::Config::Hydrogen(model) => match model {
            fluid::ModelConfig::IdealGas => fluid::IdealGas::hydrogen(),
            fluid::ModelConfig::RefProp => todo!(),
            fluid::ModelConfig::Fit => todo!(),
        },
        fluid::Config::Helium(model) => match model {
            fluid::ModelConfig::IdealGas => fluid::IdealGas::helium(),
            fluid::ModelConfig::RefProp => todo!(),
            fluid::ModelConfig::Fit => todo!(),
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
}
