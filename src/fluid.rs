mod fit;
mod ideal_gas;
mod refprop;

// Export all available fluid models
pub use ideal_gas::IdealGas;
use serde::Serialize;

pub trait Fluid {
    /// Return `density` in kg/m3
    ///
    /// # Arguments
    ///
    /// * `temp` - temperature (K)
    /// * `pres` - pressure (Pa)
    ///
    fn density(&self, temp: f64, pres: f64) -> f64;

    /// Return specific `enthalpy` in J/kg
    ///
    /// # Arguments
    ///
    /// * `temp` - temperature (K)
    /// * `pres` - pressure (Pa)
    ///
    fn enthalpy(&self, temp: f64, pres: f64) -> f64;

    /// Return the `PropSetOne` collection of properties
    ///
    /// # Arguments
    ///
    /// * `temp` - temperature (K)
    /// * `pres` - pressure (Pa)
    ///
    fn prop_set_1(&self, temp: f64, pres: f64) -> PropSetOne;

    /// Return the `PropSetTwo` collection of properties
    ///
    /// # Arguments
    ///
    /// * `temp` - temperature (K)
    /// * `pres` - pressure (Pa)
    ///
    fn prop_set_2(&self, temp: f64, pres: f64) -> PropSetTwo;

    /// Return the `PropSetThree` collection of properties
    ///
    /// # Arguments
    ///
    /// * `temp` - temperature (K)
    /// * `pres` - pressure (Pa)
    ///
    fn prop_set_3(&self, temp: f64, pres: f64) -> PropSetThree;
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct PropSetOne {
    pub dens: f64,    // density (kg/m3)
    pub inte: f64,    // specific internal energy (J/kg)
    pub enth: f64,    // specific enthalpy (J/kg)
    pub dd_dP_T: f64, // derivative of density wrt pressure at constant temperature (kg/Pa)
    pub dd_dT_P: f64, // derivative of density wrt temperature at constant pressure (kg/K)
    pub du_dP_T: f64, // derivative of internal energy wrt pressure at constant temperature (J/kg-Pa)
    pub du_dT_P: f64, // derivative of internal energy wrt pressure at constant temperature (J/kg-K)
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct PropSetTwo {
    pub dens: f64,    // density (kg/m3)
    pub inte: f64,    // specific internal energy (J/kg)
    pub enth: f64,    // specific enthalpy (J/kg)
    pub dd_dP_T: f64, // derivative of density wrt pressure at constant temperature (kg/Pa)
    pub du_dP_T: f64, // derivative of internal energy wrt pressure at constant temperature (J/kg-Pa)
}

#[derive(Debug, Serialize)]
pub struct PropSetThree {
    pub dens: f64, // density (kg/m3)
    pub cp: f64,   // specific heat at constant pressure (J/kg-K)
}
