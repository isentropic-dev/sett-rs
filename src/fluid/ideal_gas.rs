use super::{HeatExchangerProps, WorkingFluid, WorkingSpaceProps};

pub struct IdealGas {
    name: String,
}

impl IdealGas {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl WorkingFluid for IdealGas {
    fn get_ws_props(&self, _temp: f64, _pres: f64) -> WorkingSpaceProps {
        todo!()
    }

    fn get_hxr_props(&self, _temp: f64, _pres: f64) -> HeatExchangerProps {
        todo!()
    }

    fn enthalpy(&self, _temp: f64, _pres: f64) -> f64 {
        todo!()
    }

    fn report(&self) -> String {
        format!("{} (Ideal Gas)", self.name)
    }
}
