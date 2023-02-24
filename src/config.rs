struct Config {
    engine: Engine,
    solver: Solver,
    conditions: Conditions,
}

struct Engine {
    fluid: Fluid,
    components: Components,
}
struct Fluid {}
struct Components {
    chx: ColdHeatExchanger,
    hhx: HotHeatExchanger,
    regenerator: Regenerator,
    working_spaces: WorkingSpaces,
}
struct ColdHeatExchanger {}
struct HotHeatExchanger {}
struct Regenerator {}
struct WorkingSpaces {}

struct Solver {
    inner_loop: InnerLoop,
    outer_loop: OuterLoop,
    ode: OrdinaryDifferentialEquation,
}
struct InnerLoop {
    tolerance: Tolerance,
    ode_tolerance: Tolerance,
    max_iterations: u32,
}
struct OuterLoop {
    tolerance: Tolerance,
    max_iterations: u32,
}
struct OrdinaryDifferentialEquation {
    tolerance: Tolerance,
    num_timesteps: u32,
}
struct Tolerance {
    abs: f64,
    rel: f64,
}

#[allow(non_snake_case)]
struct Conditions {
    T_cold: f64,
    T_hot: f64,
    P_0: f64,
}
