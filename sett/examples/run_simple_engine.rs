use sett::{
    chx, fluid, hhx, regen,
    types::{ConvergenceTolerance, LoopTolerance, MaxIters, OdeTolerance, RunInputs, RunSettings},
    ws::{self, sinusoidal_drive::Geometry, Parasitics, ThermalResistance},
    Components, Engine, LuSolver,
};

fn main() {
    let step_size = 5.;
    let start = 500.;
    for step in 0..101 {
        let temp_source = start + step_size * f64::from(step);

        let components = Components {
            ws: Box::new(ws::SinusoidalDrive {
                frequency: 70., // 4,200 rpm
                phase_angle: 90.0,
                comp_geometry: Geometry {
                    clearance_volume: 2e-5,
                    swept_volume: 5e-4,
                },
                exp_geometry: Geometry {
                    clearance_volume: 2e-5,
                    swept_volume: 5e-4,
                },
                thermal_resistance: ThermalResistance::default(),
                parasitics: Parasitics::default(),
            }),
            chx: Box::<chx::FixedApproach>::default(),
            regen: Box::<regen::FixedApproach>::default(),
            hhx: Box::<hhx::FixedApproach>::default(),
        };
        let fluid = fluid::IdealGas::hydrogen();
        let inputs = RunInputs {
            pres_zero: 10e6,
            temp_sink: 300.,
            temp_source,
        };
        let settings = RunSettings {
            resolution: 30,
            loop_tol: LoopTolerance {
                inner: ConvergenceTolerance {
                    abs: 1e-2,
                    rel: 1e-4,
                },
                outer: ConvergenceTolerance {
                    abs: 1e-2,
                    rel: 1e-4,
                },
            },
            ode_tol: OdeTolerance {
                abs: 1e-6,
                rel: 1e-6,
            },
            max_iters: MaxIters {
                inner: 20,
                outer: 20,
            },
        };

        let engine = Engine::run::<LuSolver>(components, fluid, inputs, settings)
            .expect("engine should converge");

        println!("-----------------");
        println!("T_hot: {temp_source:?}");
        println!("time:  {:?}", engine.values.time);
        println!("P:     {:?}", engine.values.P);
        println!("T_c:   {:?}", engine.values.T_c);
        println!("T_e:   {:?}", engine.values.T_e);
    }
}
