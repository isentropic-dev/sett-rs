use std::fmt;

use super::WorkingFluid;

#[derive(Debug)]
pub struct IdealGas {
    name: String,
}

impl IdealGas {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl WorkingFluid for IdealGas {}

impl fmt::Display for IdealGas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} (Ideal Gas)", self.name)
    }
}
