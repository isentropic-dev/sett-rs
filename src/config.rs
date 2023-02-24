mod conditions;
mod engine;
mod solver;

use crate::config::{conditions::Conditions, engine::Engine, solver::Solver};

struct Config {
    engine: Engine,
    solver: Solver,
    conditions: Conditions,
}
