mod run;
mod state;

use anyhow::{bail, Result};

use crate::{
    chx,
    fluid::Fluid,
    hhx, regen,
    state_equations::{Cycle, MatrixDecomposition, SteadyStateInputs},
    types::{RunInputs, RunSettings},
    ws,
};

pub use state::Pressure;

/// Represents a Stirling engine running at cyclic steady state
pub struct Engine<T: Fluid> {
    pub components: Components,
    pub state: state::State<T>,
    pub values: state::Values,
}

/// The components of a Stirling engine
pub struct Components {
    pub ws: Box<dyn ws::WorkingSpaces>,
    pub chx: Box<dyn chx::ColdHeatExchanger>,
    pub regen: Box<dyn regen::Regenerator>,
    pub hhx: Box<dyn hhx::HotHeatExchanger>,
}

impl<T: Fluid> Engine<T> {
    /// Attempt to create a running `Engine`
    ///
    /// TODO: <https://github.com/isentropic-dev/sett-rs/issues/9>
    ///
    /// # Errors
    ///
    /// Will return `Err` if a converged engine cannot be created.
    pub fn run<U: MatrixDecomposition>(
        components: Components,
        fluid: T,
        inputs: RunInputs,
        settings: RunSettings,
    ) -> Result<Self> {
        let mut state = state::State::new_hint(&components, fluid, inputs);
        for _ in 0..settings.max_iters.outer {
            let run: run::Run<T, U> = run::Run::new(&components, &state);
            let values = run.find_steady_state(SteadyStateInputs {
                pres_zero: state.pres.t_zero,
                temp_comp_hint: state.temp.chx,
                temp_exp_hint: state.temp.hhx,
                num_points: settings.resolution,
                ode_tol: settings.ode_tol,
                conv_tol: settings.loop_tol.inner,
                max_iters: settings.max_iters.inner,
            })?;
            let values = values.into(); // convert state equation values to engine values
            match state.update(&components, &values, settings.loop_tol.outer) {
                Ok(new_state) => {
                    state = new_state;
                }
                Err(state) => {
                    return Ok(Engine {
                        components,
                        state,
                        values,
                    });
                }
            };
        }

        bail!("not converged")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        fluid::IdealGas,
        state_equations::LuSolver,
        types::{ConvergenceTolerance, LoopTolerance, MaxIters, OdeTolerance},
        ws::{sinusoidal_drive::Geometry, Parasitics, ThermalResistance},
    };

    use super::*;

    fn chx_fixed_approach() -> Box<chx::FixedApproach> {
        Box::<chx::FixedApproach>::default()
    }

    fn hhx_fixed_approach() -> Box<hhx::FixedApproach> {
        Box::<hhx::FixedApproach>::default()
    }

    fn regen_fixed_approach() -> Box<regen::FixedApproach> {
        Box::<regen::FixedApproach>::default()
    }

    fn ws_sinusoidal() -> Box<ws::SinusoidalDrive> {
        Box::new(ws::SinusoidalDrive {
            frequency: 4000. / 60., // 4,000 rpm
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
        })
    }

    #[test]
    fn run_simple_engine() {
        let components = Components {
            ws: ws_sinusoidal(),
            chx: chx_fixed_approach(),
            regen: regen_fixed_approach(),
            hhx: hhx_fixed_approach(),
        };
        let fluid = IdealGas::hydrogen();
        let inputs = RunInputs {
            pres_zero: 10e6,
            temp_sink: 300.,
            temp_source: 900.,
        };
        let settings = RunSettings {
            resolution: 30,
            loop_tol: LoopTolerance {
                inner: ConvergenceTolerance {
                    abs: 1e-3,
                    rel: 1e-6,
                },
                outer: ConvergenceTolerance {
                    abs: 1e-3,
                    rel: 1e-6,
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
        let _engine = Engine::run::<LuSolver>(components, fluid, inputs, settings)
            .expect("engine should converge");
    }
}
