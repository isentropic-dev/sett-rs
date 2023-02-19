use anyhow::{bail, Result};

use super::{Fluid, PropSetOne, PropSetThree, PropSetTwo};

pub struct IdealGas {
    name: String,
    gas_constant: f64, // gas constant R in J/kg-K
    ref_temp: f64,
    ref_enth: f64,
    cp_coefs: [f64; 6],
    enth_coefs: [f64; 6],
}

impl IdealGas {
    pub fn new(name: &str) -> Result<Self> {
        let IdealGasParameters {
            gas_constant,
            ref_temp,
            cp_coefs,
        } = match name {
            "helium" => IdealGasParameters {
                gas_constant: 2077.23,
                ref_temp: 250.,
                cp_coefs: [5193.17, 0., 0., 0., 0., 0.],
            },
            "hydrogen" => IdealGasParameters {
                gas_constant: 4124.2,
                ref_temp: 250.,
                cp_coefs: [
                    12471.4839,
                    13.3194432,
                    -0.0347782806,
                    0.0000431784838,
                    -2.42883335e-08,
                    5.14289838e-12,
                ],
            },
            _ => bail!("unknown ideal gas '{name}'"),
        };

        // Enthalpy coefficients come from integration of cp coefficients
        let enth_coefs = [
            cp_coefs[0],
            cp_coefs[1] / 2.,
            cp_coefs[2] / 3.,
            cp_coefs[3] / 4.,
            cp_coefs[4] / 5.,
            cp_coefs[5] / 6.,
        ];

        Ok(Self {
            name: name.into(),
            gas_constant,
            ref_temp,
            ref_enth: poly(enth_coefs, ref_temp),
            cp_coefs,
            enth_coefs,
        })
    }
}

impl Fluid for IdealGas {
    fn density(&self, temp: f64, pres: f64) -> f64 {
        pres / (self.gas_constant * temp)
    }

    fn enthalpy(&self, temp: f64, _pres: f64) -> f64 {
        poly(self.enth_coefs, temp) - self.ref_enth
    }

    #[allow(non_snake_case)]
    fn prop_set_1(&self, temp: f64, pres: f64) -> PropSetOne {
        let dens = self.density(temp, pres);
        let cp = poly(self.cp_coefs, temp);
        let cv = cp - self.gas_constant;
        let enth = self.enthalpy(temp, pres);
        let inte = enth - self.gas_constant * (temp - self.ref_temp);
        let dd_dT_P = -pres / (self.gas_constant * temp.powi(2));
        let dd_dP_T = 1. / (self.gas_constant * temp);
        let du_dT_P = cv;
        let du_dP_T = 0.;
        PropSetOne {
            dens,
            inte,
            enth,
            dd_dP_T,
            dd_dT_P,
            du_dP_T,
            du_dT_P,
        }
    }

    #[allow(non_snake_case)]
    fn prop_set_2(&self, temp: f64, pres: f64) -> PropSetTwo {
        let dens = self.density(temp, pres);
        let enth = self.enthalpy(temp, pres);
        let inte = enth - self.gas_constant * (temp - self.ref_temp);
        let dd_dP_T = 1. / (self.gas_constant * temp);
        let du_dP_T = 0.;
        PropSetTwo {
            dens,
            inte,
            enth,
            dd_dP_T,
            du_dP_T,
        }
    }

    fn prop_set_3(&self, temp: f64, pres: f64) -> PropSetThree {
        let dens = self.density(temp, pres);
        let cp = poly(self.cp_coefs, temp);
        PropSetThree { dens, cp }
    }
}

struct IdealGasParameters {
    gas_constant: f64,
    ref_temp: f64,
    cp_coefs: [f64; 6],
}

/// Evaluate a 5th order polynomial using Horner's method
///
/// Polynomial format is `a[0] + a[1]*x + a[2]*x^2 + a[3]*x^3 + a[4]*x^4 + a[5]*x^5`
#[inline(always)]
fn poly(a: [f64; 6], x: f64) -> f64 {
    ((((a[5] * x + a[4]) * x + a[3]) * x + a[2]) * x + a[1]) * x + a[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_poly() {
        let coefs = [1., 2.5, 3., 2.2, 1.3, 5.];
        assert_eq!(poly(coefs, 0.), 1.);
        assert_eq!(poly(coefs, 0.5), 3.5125);
        assert_eq!(poly(coefs, 1.), 15.);
    }

    #[test]
    fn helium() {
        let fluid = IdealGas::new("helium").expect("should work");
        let PropSetTwo { inte, enth, .. } = fluid.prop_set_2(fluid.ref_temp, 101.);
        assert_eq!(enth, 0., "enth should be zero at reference temperature");
        assert_eq!(inte, 0., "inte should be zero at reference temperature");
        insta::assert_yaml_snapshot!(fluid.prop_set_1(500., 10e6), @r###"
        ---
        dens: 9.628206794625536
        inte: -519307.5
        enth: 0
        dd_dP_T: 0.0000009628206794625535
        dd_dT_P: -0.01925641358925107
        du_dP_T: 0
        du_dT_P: 3115.94
        "###);
        insta::assert_yaml_snapshot!(fluid.prop_set_3(450., 5000.), @r###"
        ---
        dens: 0.005349003774791964
        cp: 5193.17
        "###);
    }

    #[test]
    fn hydrogen() {
        let fluid = IdealGas::new("hydrogen").expect("should work");
        let PropSetTwo { inte, enth, .. } = fluid.prop_set_2(fluid.ref_temp, 101.);
        assert_eq!(enth, 0., "enth should be zero at reference temperature");
        assert_eq!(inte, 0., "inte should be zero at reference temperature");
        insta::assert_yaml_snapshot!(fluid.prop_set_1(500., 10e6), @r###"
        ---
        dens: 4.849425343096843
        inte: -1030636.7305105176
        enth: 413.2694894824217
        dd_dP_T: 0.0000004849425343096843
        dd_dT_P: -0.009698850686193685
        du_dP_T: 0
        du_dT_P: 10352.440555624999
        "###);
        insta::assert_yaml_snapshot!(fluid.prop_set_3(450., 5000.), @r###"
        ---
        dens: 0.002694125190609357
        cp: 14456.198318703318
        "###);
    }
}
