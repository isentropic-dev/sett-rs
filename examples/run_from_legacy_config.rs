use sett_rs::LegacyConfig;

fn main() {
    let config_str = r#"

{
                "fluid": {
                    "model": "Hydrogen"
                },
                "ws": {
                    "model": "Sinusoidal",
                    "params": {
                        "frequency": 70,
                        "phase_angle": 90,
                        "V_swept_c": 5e-4,
                        "V_clearance_c": 2e-5,
                        "R_c": "Inf",
                        "W_parasitic_c": 0,
                        "V_swept_e": 5e-4,
                        "V_clearance_e": 2e-5,
                        "R_e": "Inf",
                        "W_parasitic_e": 0,
                        "Q_parasitic_e": 0
                    } 
                },
                "chx": {
                    "model": "FixedApproach",
                    "params": {
                        "vol": 4e-5,
                        "DT": 40,
                        "R_hyd": 0,
                        "W_parasitic": 0
                    }
                },
                "regen": {
                    "model": "FixedApproach",
                    "params": {
                        "vol": 1e-4,
                        "DT": 10,
                        "R_hyd": 0,
                        "Q_parasitic": 0
                    }
                
                },
                "hhx": {
                    "model": "FixedApproach",
                    "params": {
                        "vol": 1e-4,
                        "DT": 100,
                        "R_hyd": 0,
                        "W_parasitic": 0,
                        "Q_parasitic": 0
                    }
                },
                "solver": {
                    "innerLoopTolerance": {
                        "abs": 1e-2,
                        "rel": 1e-4
                    },
                    "odeSolver": "ode45",
                    "odeTolerance": {
                        "abs": 1e-6,
                        "rel": 1e-6
                    },
                    "outerLoopTolerance": {
                        "abs": 1e-4,
                        "rel": 1e-4
                    },
                    "timeResolution": 30
                },
                "conditions": {
                    "T_cold": 300,
                    "T_hot": 500,
                    "P_0": 10e6
                }
            }
            "#;

    let config = config::Config::builder()
        .add_source(config::File::from_str(config_str, config::FileFormat::Json))
        .build()
        .unwrap()
        .try_deserialize::<LegacyConfig>()
        .unwrap();

    sett_rs::run_from_legacy_config(config);
}
