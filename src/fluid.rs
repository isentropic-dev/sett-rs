mod fit;
mod ideal_gas;
mod refprop;

// Export all available fluid models
pub use ideal_gas::IdealGas;
use serde::Deserialize;

pub trait Fluid {
    /// Return density in kg/m3
    ///
    /// # Arguments
    ///
    /// * `temp` - temperature (K)
    /// * `pres` - pressure (Pa)
    ///
    fn dens(&self, temp: f64, pres: f64) -> f64;

    /// Return specific internal energy in J/kg
    ///
    /// # Arguments
    ///
    /// * `temp` - temperature (K)
    /// * `pres` - pressure (Pa)
    ///
    fn inte(&self, temp: f64, pres: f64) -> f64;

    /// Return specific enthalpy in J/kg
    ///
    /// # Arguments
    ///
    /// * `temp` - temperature (K)
    /// * `pres` - pressure (Pa)
    ///
    fn enth(&self, temp: f64, pres: f64) -> f64;

    /// Return specific heat at constant pressure in J/kg-K
    ///
    /// # Arguments
    ///
    /// * `temp` - temperature (K)
    /// * `pres` - pressure (Pa)
    ///
    fn cp(&self, temp: f64, pres: f64) -> f64;

    /// Return derivative of density with respect to pressure at constant temperature in kg/m3-Pa
    ///
    /// # Arguments
    ///
    /// * `temp` - temperature (K)
    /// * `pres` - pressure (Pa)
    ///
    #[allow(non_snake_case)]
    fn dd_dP_T(&self, temp: f64, pres: f64) -> f64;

    /// Return derivative of density with respect to temperature at constant pressure in kg/m3-K
    ///
    /// # Arguments
    ///
    /// * `temp` - temperature (K)
    /// * `pres` - pressure (Pa)
    ///
    #[allow(non_snake_case)]
    fn dd_dT_P(&self, temp: f64, pres: f64) -> f64;

    /// Return derivative of internal energy with respect to pressure at constant temperature in J/kg-Pa
    ///
    /// # Arguments
    ///
    /// * `temp` - temperature (K)
    /// * `pres` - pressure (Pa)
    ///
    #[allow(non_snake_case)]
    fn du_dP_T(&self, temp: f64, pres: f64) -> f64;

    /// Return derivative of internal energy with respect to temperature at constant pressure in J/kg-K
    ///
    /// # Arguments
    ///
    /// * `temp` - temperature (K)
    /// * `pres` - pressure (Pa)
    ///
    #[allow(non_snake_case)]
    fn du_dT_P(&self, temp: f64, pres: f64) -> f64;
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "name", content = "model")]
pub enum FluidConfig {
    Hydrogen(FluidModelConfig),
    Helium(FluidModelConfig),
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FluidModelConfig {
    IdealGas,
    RefProp,
    FIT,
}

impl FluidConfig {
    pub fn into(&self) -> impl Fluid {
        match self {
            FluidConfig::Hydrogen(model) => match model {
                FluidModelConfig::IdealGas => IdealGas::hydrogen(),
                FluidModelConfig::RefProp => todo!(),
                FluidModelConfig::FIT => todo!(),
            },
            FluidConfig::Helium(model) => match model {
                FluidModelConfig::IdealGas => IdealGas::helium(),
                FluidModelConfig::RefProp => todo!(),
                FluidModelConfig::FIT => todo!(),
            },
        }
    }
}
