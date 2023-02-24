pub(super) struct Solver {
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
