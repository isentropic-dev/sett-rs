pub trait WorkingFluid {
    /// Provide a short description of the working fluid
    fn describe(&self) -> String;
}

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

impl WorkingFluid for IdealGas {
    fn describe(&self) -> String {
        format!("Ideal Gas {}", self.name)
    }
}
