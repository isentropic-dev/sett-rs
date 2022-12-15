use na::Vector4;
use ode_solvers::{Rk4, System};

type Time = f64;
// State[0] = Velocity of mass 1.
// State[1] = Position of mass 1.
// State[2] = Velocity of mass 2.
// State[3] = Position of mass 2.
type State = Vector4<f64>;

// A two degree of freedom mass-spring system. Single spring constant and equal masses.
struct MassSpringSystem {
    k: f64,
    m: f64,
}

impl System<State> for MassSpringSystem {
    fn system(&self, _: Time, y: &State, dy: &mut State) {
        dy[0] = (-2. * self.k * y[1]) / self.m + (self.k * y[3]) / self.m;
        dy[1] = y[0];
        dy[2] = (self.k * y[1]) / self.m - (2. * self.k * y[3]) / self.m;
        dy[3] = y[2];
    }
}

fn main() {
    // Initial values.
    let y0 = State::new(0., 5., 0., 10.);

    // Define the system.
    let system = MassSpringSystem { k: 1., m: 5. };

    // Run the integration.
    let mut stepper = Rk4::new(system, 0., y0, 10., 0.25);
    let results = stepper.integrate().expect("No dice integrating.");

    println!("{}", results);
    for (x, ys) in stepper.x_out().iter().zip(stepper.y_out().iter()) {
        println!(
            "time: {:?}, v1: {:.3?}, x1: {:.3?}, v2: {:.3?}, x2: {:.3?}",
            x, ys[0], ys[1], ys[2], ys[3]
        );
    }
}
