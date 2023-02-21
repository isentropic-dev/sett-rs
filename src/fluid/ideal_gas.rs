use super::Fluid;

pub struct IdealGas {
    name: Name,
    gas_constant: f64, // gas constant R in J/kg-K
    ref_temp: f64,
    cp_coefs: [f64; 6],
    enth_coefs: [f64; 6],
}

/// The available ideal gas fluids
pub enum Name {
    Helium,
    Hydrogen,
}

impl IdealGas {
    /// Return an ideal gas model for helium
    pub fn helium() -> Self {
        Self::new(Name::Helium)
    }

    /// Return an ideal gas model for hydrogen
    pub fn hydrogen() -> Self {
        Self::new(Name::Hydrogen)
    }

    /// Return an ideal gas model for `name`
    pub fn new(name: Name) -> Self {
        let IdealGasParameters {
            gas_constant,
            ref_temp,
            cp_coefs,
        } = match name {
            Name::Helium => IdealGasParameters {
                gas_constant: 2077.23,
                ref_temp: 250.,
                cp_coefs: [5193.17, 0., 0., 0., 0., 0.],
            },
            Name::Hydrogen => IdealGasParameters {
                gas_constant: 4124.2,
                ref_temp: 250.,
                cp_coefs: [
                    12471.4839,
                    13.319_443_2,
                    -3.477_828_06e-2,
                    4.317_848_38e-5,
                    -2.428_833_35e-8,
                    5.142_898_38e-12,
                ],
            },
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

        Self {
            name,
            gas_constant,
            ref_temp,
            cp_coefs,
            enth_coefs,
        }
    }
}

impl Fluid for IdealGas {
    fn dens(&self, temp: f64, pres: f64) -> f64 {
        pres / (self.gas_constant * temp)
    }

    fn inte(&self, temp: f64, pres: f64) -> f64 {
        let enth = self.enth(temp, pres);
        enth - self.gas_constant * (temp - self.ref_temp)
    }

    fn enth(&self, temp: f64, _pres: f64) -> f64 {
        // Need to adjust coefficients based on the difference from reference
        let ref_diff = temp - self.ref_temp;
        let coefs = self.enth_coefs.map(|x| x * ref_diff);
        poly(coefs, temp)
    }

    fn cp(&self, temp: f64, _pres: f64) -> f64 {
        poly(self.cp_coefs, temp)
    }

    fn dd_dP_T(&self, temp: f64, _pres: f64) -> f64 {
        1. / (self.gas_constant * temp)
    }

    fn dd_dT_P(&self, temp: f64, pres: f64) -> f64 {
        -pres / (self.gas_constant * temp.powi(2))
    }

    fn du_dP_T(&self, _temp: f64, _pres: f64) -> f64 {
        0.
    }

    fn du_dT_P(&self, temp: f64, pres: f64) -> f64 {
        let cp = self.cp(temp, pres);
        cp - self.gas_constant
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
fn poly(a: [f64; 6], x: f64) -> f64 {
    ((((a[5] * x + a[4]) * x + a[3]) * x + a[2]) * x + a[1]) * x + a[0]
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::*;

    #[allow(non_snake_case)]
    #[derive(Serialize)]
    struct AllProps {
        dens: f64,
        inte: f64,
        enth: f64,
        cp: f64,
        dd_dP_T: f64,
        dd_dT_P: f64,
        du_dP_T: f64,
        du_dT_P: f64,
    }

    impl AllProps {
        /// Return all the properties for `name` at a given `temp` and `pres`
        fn new(name: Name, temp: f64, pres: f64) -> Self {
            let fluid = IdealGas::new(name);
            Self {
                dens: fluid.dens(temp, pres),
                inte: fluid.inte(temp, pres),
                enth: fluid.enth(temp, pres),
                cp: fluid.cp(temp, pres),
                dd_dP_T: fluid.dd_dP_T(temp, pres),
                dd_dT_P: fluid.dd_dT_P(temp, pres),
                du_dP_T: fluid.du_dP_T(temp, pres),
                du_dT_P: fluid.du_dT_P(temp, pres),
            }
        }
    }

    /// Fails if enthalpy and internal energy aren't 0 a the reference temperature
    fn check_at_reference(name: Name) {
        let fluid = IdealGas::new(name);
        let temp = fluid.ref_temp;
        let pres = 101e3; // arbitrarily use atmospheric pressure
        assert_eq!(
            fluid.enth(temp, pres),
            0.,
            "enth should be zero at reference temperature"
        );
        assert_eq!(
            fluid.inte(temp, pres),
            0.,
            "inte should be zero at reference temperature"
        );
    }

    #[test]
    fn check_poly() {
        let coefs = [1., 2.5, 3., 2.2, 1.3, 5.];
        assert_eq!(poly(coefs, 0.), 1.);
        assert_eq!(poly(coefs, 0.5), 3.5125);
        assert_eq!(poly(coefs, 1.), 15.);
    }

    #[test]
    fn helium() {
        check_at_reference(Name::Helium);
        insta::assert_yaml_snapshot!(AllProps::new(Name::Helium, 500.0, 10e6), @r###"
        ---
        dens: 9.628206794625536
        inte: 778985
        enth: 1298292.5
        cp: 5193.17
        dd_dP_T: 0.0000009628206794625535
        dd_dT_P: -0.01925641358925107
        du_dP_T: 0
        du_dT_P: 3115.94
        "###);
    }

    #[test]
    fn hydrogen() {
        check_at_reference(Name::Hydrogen);
        insta::assert_yaml_snapshot!(AllProps::new(Name::Hydrogen, 500.0, 10e6), @r###"
        ---
        dens: 4.849425343096843
        inte: 2462866.0072656246
        enth: 3493916.0072656246
        cp: 14476.640555625
        dd_dP_T: 0.0000004849425343096843
        dd_dT_P: -0.009698850686193685
        du_dP_T: 0
        du_dT_P: 10352.440555624999
        "###);
    }
}
