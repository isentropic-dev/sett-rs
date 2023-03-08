use sett::Config;

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
            temp_sink = 300
            temp_source = 500
            pres_zero = 10e6
            "#;

    let config = config::Config::builder()
        .add_source(config::File::from_str(config_str, config::FileFormat::Toml))
        .build()
        .unwrap()
        .try_deserialize::<Config>()
        .unwrap();

    sett::run_from_config(config);
}
