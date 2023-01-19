use std::fmt;

use super::WorkingFluid;

#[derive(Debug)]
pub struct IdealGas {
    name: String,
}

impl IdealGas {
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl WorkingFluid for IdealGas {}

impl fmt::Display for IdealGas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} (Ideal Gas)", self.name)
    }
}
