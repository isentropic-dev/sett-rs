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

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Config {
    Hydrogen(ModelConfig),
    Helium(ModelConfig),
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", tag = "model")]
pub enum ModelConfig {
    Custom,
    Fit,
    IdealGas,
    RefProp,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "model", content = "params")]
pub enum LegacyConfig {
    Hydrogen,
    RealGasRefprop(LegacyRefPropParams),
    IdealGas(LegacyIdealGasParams),
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct LegacyRefPropParams {
    name: LegacyFluidOption,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct LegacyIdealGasParams {
    name: LegacyFluidOption,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum LegacyFluidOption {
    Hydrogen,
    Helium,
}

impl LegacyConfig {
    #[must_use]
    pub fn into(self) -> Config {
        match self {
            LegacyConfig::Hydrogen => Config::Hydrogen(ModelConfig::Custom),
            LegacyConfig::RealGasRefprop(params) => match params.name {
                LegacyFluidOption::Hydrogen => Config::Hydrogen(ModelConfig::RefProp),
                LegacyFluidOption::Helium => Config::Helium(ModelConfig::RefProp),
            },
            LegacyConfig::IdealGas(params) => match params.name {
                LegacyFluidOption::Hydrogen => Config::Hydrogen(ModelConfig::IdealGas),
                LegacyFluidOption::Helium => Config::Helium(ModelConfig::IdealGas),
            },
        }
    }
}
