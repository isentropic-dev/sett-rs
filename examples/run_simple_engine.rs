use sett_rs::Config;

fn main() {
    let config_str = r#"
            [engine.fluid.hydrogen]
            model = "ideal_gas"

            [engine.components.chx.fixed_approach]
            vol = 4e-5
            DT = 40
            R_hyd = 0
            W_parasitic = 0

            [engine.components.hhx.fixed_approach]
            vol = 1e-4
            DT = 100
            R_hyd = 0
            W_parasitic = 0
            Q_parasitic = 0

            [engine.components.regen.fixed_approach]
            vol = 1e-4
            DT = 10
            R_hyd = 0
            Q_parasitic = 0

            [engine.components.ws.sinusoidal]
            frequency = 70
            phase_angle = 90
            V_swept_c = 5e-4
            V_clearance_c = 2e-5
            R_c = inf
            W_parasitic_c = 0
            V_swept_e = 5e-4
            V_clearance_e = 2e-5
            R_e = inf
            W_parasitic_e = 0
            Q_parasitic_e = 0
            
            [solver.inner_loop]
            tolerance = { abs = 1e-2, rel = 1e-4 }
            max_iterations = 20

            [solver.outer_loop]
            tolerance = { abs = 1e-2, rel = 1e-4 }
            max_iterations = 20

            [solver.ode]
            tolerance = { abs = 1e-6, rel = 1e-6 }
            num_timesteps = 30

            [conditions]
            T_cold = 300
            T_hot = 500
            P_0 = 10e6
            "#;

    let config = config::Config::builder()
        .add_source(config::File::from_str(config_str, config::FileFormat::Toml))
        .build()
        .unwrap()
        .try_deserialize::<Config>()
        .unwrap();

    sett_rs::run_from_config(config);

    // let step_size = 5.;
    // let start = 500.;
    // for step in 0..101 {
    //     let temp_source = start + step_size * f64::from(step);

    //     let components = Components {
    //         ws: Box::new(ws::SinusoidalDrive {
    //             frequency: 70., // 4,200 rpm
    //             phase_angle: 90.0,
    //             comp_geometry: Geometry {
    //                 clearance_volume: 2e-5,
    //                 swept_volume: 5e-4,
    //             },
    //             exp_geometry: Geometry {
    //                 clearance_volume: 2e-5,
    //                 swept_volume: 5e-4,
    //             },
    //             thermal_resistance: ThermalResistance::default(),
    //             parasitics: Parasitics::default(),
    //         }),
    //         chx: Box::<chx::FixedApproach>::default(),
    //         regen: Box::<regen::FixedApproach>::default(),
    //         hhx: Box::<hhx::FixedApproach>::default(),
    //     };
    //     let fluid = fluid::IdealGas::hydrogen();
    //     let inputs = RunInputs {
    //         pres_zero: 10e6,
    //         temp_sink: 300.,
    //         temp_source,
    //     };
    //     let settings = RunSettings {
    //         resolution: 30,
    //         loop_tol: LoopTolerance {
    //             inner: ConvergenceTolerance {
    //                 abs: 1e-2,
    //                 rel: 1e-4,
    //             },
    //             outer: ConvergenceTolerance {
    //                 abs: 1e-2,
    //                 rel: 1e-4,
    //             },
    //         },
    //         ode_tol: OdeTolerance {
    //             abs: 1e-6,
    //             rel: 1e-6,
    //         },
    //         max_iters: MaxIters {
    //             inner: 20,
    //             outer: 20,
    //         },
    //     };

    //     let engine = Engine::run::<LuSolver>(components, fluid, inputs, settings)
    //         .expect("engine should converge");

    //     println!("-----------------");
    //     println!("T_hot: {temp_source:?}");
    //     println!("time:  {:?}", engine.values.time);
    //     println!("P:     {:?}", engine.values.P);
    //     println!("T_c:   {:?}", engine.values.T_c);
    //     println!("T_e:   {:?}", engine.values.T_e);
    // }
}
