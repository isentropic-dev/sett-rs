use crate::engine::state;

/// The results of an engine run
///
/// The properties with type `Vec<f64>` represent discrete values in time
/// with a common index.  For example, at `time[5]` the compression space
/// temprerature is `T_c[5]` and the mass flow rate between the chx and the
/// regenerator is `m_dot_kr[5]`.
///
/// For mass flow rate values, positive values indicate flow from cold to hot
/// (i.e., positive flow is comp -> chx -> regen -> hhx -> exp).  Negative
/// values represent mass flow in the opposite direction.
#[allow(non_snake_case)]
pub struct RunResults {
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

    /// Engine pressure (Pa)
    pub pres: state::Pressure,

    /// Engine temperatures (K)
    pub temp: state::Temperatures,

    /// Average mass flow rates through the heat exchangers (kg/s)
    pub mass_flow: state::MassFlows,

    /// Average heat flow rates through the heat exhangers (W)
    pub heat_flow: state::HeatFlows,

    /// Regenerator approach temperature imbalance (K)
    pub regen_imbalance: state::RegenImbalance,

    /// Total heat input to the engine (W)
    pub heat_input: f64,

    /// Total heat rejection from the engine (W)
    pub heat_rejection: f64,

    /// Indicated power, assuming no hxr pressure drop (W)
    pub indicated_power_ideal: f64,

    /// Indicated power, accounting for hxr pressure drop (W)
    pub indicated_power: f64,

    /// Shaft power (W)
    ///
    /// Shaft power is defined as the indicated power less any mechanical
    /// parasitics in the working spaces.
    pub shaft_power: f64,

    /// Shaft torque (N-m)
    pub shaft_torque: f64,

    /// Net power (W)
    ///
    /// Net power is defined as the shaft power less any mechanical parasitics
    /// in the heat exchangers.
    pub net_power: f64,

    /// Engine efficiency, ignoring electrical parasitics (-)
    pub eta_mechanical: f64,

    /// Engine efficiency, including electrical parasitics (-)
    pub eta_overall: f64,
}
