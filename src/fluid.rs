mod fit;
mod ideal_gas;
mod refprop;

// Export all available fluid models
pub use ideal_gas::IdealGas;

#[allow(clippy::module_name_repetitions)]
pub trait WorkingFluid {
    /// Returns properties required by state equations related to working spaces
    fn get_ws_props(&self, temp: f64, pres: f64) -> WorkingSpaceProps;

    /// Returns properties required by state equations related to heat exchangers
    fn get_hxr_props(&self, temp: f64, pres: f64) -> HeatExchangerProps;

    /// Returns enthalpy
    fn enthalpy(&self, temp: f64, pres: f64) -> f64;

    /// Returns information about the fluid model
    fn report(&self) -> String;
}

#[allow(non_snake_case)]
pub struct WorkingSpaceProps {
    pub dens: f64,
    pub inte: f64,
    pub enth: f64,
    pub dd_dP_T: f64,
    pub dd_dT_P: f64,
    pub du_dP_T: f64,
    pub du_dT_P: f64,
}

#[allow(non_snake_case)]
pub struct HeatExchangerProps {
    pub dens: f64,
    pub inte: f64,
    pub enth: f64,
    pub dd_dP_T: f64,
    pub du_dP_T: f64,
}
