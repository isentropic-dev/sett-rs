use crate::{
    config::Config,
    fluid::{self, Fluid},
    performance::Performance,
    state_equations::LuSolver,
    types::RunError,
    Engine,
};

/// The main interface for running an engine
///
/// # Errors
///
/// Will return `Err` if the inner loop or outer loop convergence fails.
///
/// # Panics
///
/// Will panic if an unsupported fluid model is provided.  When <https://
/// github.com/ isentropic-dev/sett-rs/issues/63> is resolved, the function
/// will no longer panic.
pub fn run_engine(config: Config) -> Result<RunResults, RunError> {
    let fluid = match config.engine.fluid {
        fluid::Config::Hydrogen(model) => match model {
            fluid::ModelConfig::IdealGas => fluid::IdealGas::hydrogen(),
            fluid::ModelConfig::RefProp => todo!(),
            fluid::ModelConfig::Fit => todo!(),
            fluid::ModelConfig::Custom => todo!(),
        },
        fluid::Config::Helium(model) => match model {
            fluid::ModelConfig::IdealGas => fluid::IdealGas::helium(),
            fluid::ModelConfig::RefProp => todo!(),
            fluid::ModelConfig::Fit => todo!(),
            fluid::ModelConfig::Custom => todo!(),
        },
    };

    let engine = Engine::run::<LuSolver>(
        config.engine.components.into(),
        fluid,
        config.conditions.into(),
        config.solver.into(),
    )?;

    Ok(RunResults::from(engine))
}

/// The results of an engine run
#[derive(Debug)]
pub struct RunResults {
    /// Engine efficiency (-)
    pub efficiency: Efficiency,

    /// Average heat flow rates (W)
    pub heat_flow: HeatFlow,

    /// Average mass flow rates through the heat exchangers (kg/s)
    pub mass_flow: MassFlow,

    /// Engine power (W)
    pub power: Power,

    /// Engine pressure (Pa)
    pub pressure: Pressure,

    /// Regenerator approach temperature imbalance (K)
    pub regen_imbalance: f64,

    /// Shaft torque (N-m)
    pub shaft_torque: f64,

    /// Engine temperatures (K)
    pub temperature: Temperature,

    /// Time-discretized values over one engine cycle
    pub values: Values,
}

/// Different characterizations of engine efficiency
#[derive(Debug)]
pub struct Efficiency {
    /// Mechanical efficiency, which ignores electrical parasitics
    pub mechanical: f64,

    /// Overal efficiency, which includes electrical parasitics
    pub overall: f64,
}

/// Average heat flow rates (W)
#[derive(Debug)]
pub struct HeatFlow {
    /// Total heat input to the engine
    pub input: f64,

    /// Total heat rejection from the engine
    pub rejection: f64,

    /// Heat flow through the cold heat exchanger
    pub chx: f64,

    /// Heat flow into the regenerator
    pub regen: f64,

    /// Heat flow through the hot heat exchanger
    pub hhx: f64,
}

/// Average mass flow rates through the heat exchangers (kg/s)
#[derive(Debug)]
pub struct MassFlow {
    pub chx: f64,
    pub regen: f64,
    pub hhx: f64,
}

/// Different characterizations of engine power
#[derive(Debug)]
pub struct Power {
    /// Ideal indicated power, which assumes no hxr pressure drop (W)
    pub ideal_indicated: f64,

    /// Indicated power, which accounts for hxr pressure drop (W)
    pub indicated: f64,

    /// Shaft power (W)
    ///
    /// Shaft power is defined as the indicated power less any mechanical
    /// parasitics in the working spaces.
    pub shaft: f64,

    /// Net power (W)
    ///
    /// Net power is defined as the shaft power less any mechanical parasitics
    /// in the heat exchangers.
    pub net: f64,
}

/// Engine pressure (Pa)
#[derive(Debug)]
pub struct Pressure {
    pub avg: f64,
    pub max: f64,
    pub min: f64,
    pub t_zero: f64,
}

/// Engine temperature (K)
#[derive(Debug)]
pub struct Temperature {
    pub sink: f64,
    pub chx: f64,
    pub regen_cold: f64,
    pub regen_avg: f64,
    pub regen_hot: f64,
    pub hhx: f64,
    pub source: f64,
}

/// Time-discretized values over one engine cycle
///
/// This struct represents discrete values in time during a single engine
/// cycle, all sharing a common index.  For example, at `time[5]` the
/// compression space temperature is `T_c[5]` and the mass flow rate between
/// the chx and the regenerator is `m_dot_kr[5]`.
///
/// For mass flow rate values, positive values indicate flow from cold to hot
/// (i.e., positive flow is comp -> chx -> regen -> hhx -> exp).  Negative
/// values represent mass flow in the opposite direction.
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Values {
    /// Time in the cycle for each discrete point (s)
    pub time: Vec<f64>,

    /// Pressure in all volumes, assuming no hxr pressure drop (Pa)
    pub P: Vec<f64>,

    /// Pressure in the compression space, accounting for hxr pressure drop (Pa)
    pub P_c: Vec<f64>,

    /// Pressure in the expansion space, accounting for hxr pressure drop (Pa)
    pub P_e: Vec<f64>,

    /// Temperature in the compression space (K)
    pub T_c: Vec<f64>,

    /// Temperature in the expansion space (K)
    pub T_e: Vec<f64>,

    /// Mass flow rate from compression space to cold heat exchanger (kg/s)
    pub m_dot_ck: Vec<f64>,

    /// Mass flow rate from cold heat exchanger to regenerator (kg/s)
    pub m_dot_kr: Vec<f64>,

    /// Mass flow rate from regenerator to hot heat exchanger (kg/s)
    pub m_dot_rl: Vec<f64>,

    /// Mass flow rate from hot heat exchanger to expansion space (kg/s)
    pub m_dot_le: Vec<f64>,

    /// Heat flow from the working fluid to the cold heat exchanger (W)
    ///
    /// Positive values represent heat flow from the fluid to the heat exchanger.
    pub Q_dot_k: Vec<f64>,

    /// Heat flow from the working fluid into the regenerator (W)
    ///
    /// Positive values represent heat flow from the fluid into the regenerator.
    pub Q_dot_r: Vec<f64>,

    /// Heat flow from the hot heat exchanger to the working fluid  (W)
    ///
    /// Positive values represent heat flow from the heat exchanger to the fluid.
    pub Q_dot_l: Vec<f64>,
}

impl<T: Fluid> From<Engine<T>> for RunResults {
    fn from(engine: Engine<T>) -> Self {
        let performance = Performance::from(&engine);

        Self {
            efficiency: Efficiency {
                mechanical: performance.efficiency,
                // TODO: How do we calculate this?
                overall: 0.,
            },
            heat_flow: HeatFlow {
                input: performance.heat.input,
                rejection: performance.heat.rejected,
                chx: engine.state.heat_flow.chx,
                regen: engine.state.heat_flow.regen,
                hhx: engine.state.heat_flow.hhx,
            },
            mass_flow: MassFlow {
                chx: engine.state.mass_flow.chx,
                regen: engine.state.mass_flow.regen,
                hhx: engine.state.mass_flow.hhx,
            },
            power: Power {
                ideal_indicated: performance.power.indicated_zero_dP,
                indicated: performance.power.indicated,
                shaft: performance.power.shaft,
                net: performance.power.net,
            },
            pressure: Pressure {
                avg: engine.state.pres.avg,
                max: engine.state.pres.max,
                min: engine.state.pres.min,
                t_zero: engine.state.pres.t_zero,
            },
            regen_imbalance: engine.state.regen_imbalance.0,
            shaft_torque: performance.shaft_torque,
            temperature: Temperature {
                sink: engine.state.temp.sink,
                chx: engine.state.temp.chx,
                regen_cold: engine.state.temp.regen.cold,
                regen_avg: engine.state.temp.regen.avg,
                regen_hot: engine.state.temp.regen.hot,
                hhx: engine.state.temp.hhx,
                source: engine.state.temp.source,
            },
            values: Values {
                time: engine.values.time,
                P: engine.values.P,
                P_c: performance.pressures_with_drops.P_c.data.into(),
                P_e: performance.pressures_with_drops.P_e.data.into(),
                T_c: engine.values.T_c,
                T_e: engine.values.T_e,
                m_dot_ck: engine.values.m_dot_ck,
                m_dot_kr: engine.values.m_dot_kr,
                m_dot_rl: engine.values.m_dot_rl,
                m_dot_le: engine.values.m_dot_le,
                Q_dot_k: engine.values.Q_dot_k,
                Q_dot_r: engine.values.Q_dot_r,
                Q_dot_l: engine.values.Q_dot_l,
            },
        }
    }
}
