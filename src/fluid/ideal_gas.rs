use super::WorkingFluid;

pub struct IdealGas {
    name: String,
}

impl IdealGas {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl WorkingFluid for IdealGas {
    fn report(&self) -> String {
        format!("{} (Ideal Gas)", self.name)
    }
}
