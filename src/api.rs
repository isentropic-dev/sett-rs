use crate::engine::state;

/// The results of an engine run
pub struct RunResults {
    /// Engine efficiency (-)
    pub efficiency: Efficiency,

    /// Engine heat (W)
    pub heat: Heat,

    /// Average heat flow rates through the heat exhangers (W)
    pub heat_flow: state::HeatFlows,

    /// Average mass flow rates through the heat exchangers (kg/s)
    pub mass_flow: state::MassFlows,

    /// Engine power (W)
    pub power: Power,

    /// Engine pressure (Pa)
    pub pres: state::Pressure,

    /// Regenerator approach temperature imbalance (K)
    pub regen_imbalance: state::RegenImbalance,

    /// Shaft torque (N-m)
    pub shaft_torque: f64,

    /// Engine temperatures (K)
    pub temp: state::Temperatures,

    /// Time-discretized values over one engine cycle
    pub values: Values,
}

/// Different characterizations of engine efficiency
pub struct Efficiency {
    /// Mechanical efficiency, which ignores electrical parasitics
    pub mechanical: f64,

    /// Overal efficiency, which includes electrical parasitics
    pub overall: f64,
}

/// Total heat input and rejection (W)
pub struct Heat {
    pub input: f64,
    pub rejection: f64,
}

/// Different characterizations of engine power
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