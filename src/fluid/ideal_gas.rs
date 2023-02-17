use super::Fluid;

pub struct IdealGas {
    name: String,
}

impl IdealGas {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl Fluid for IdealGas {}
