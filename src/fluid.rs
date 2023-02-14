mod fit;
mod ideal_gas;
mod refprop;

// Export all available fluid models
pub use ideal_gas::IdealGas;

#[allow(clippy::module_name_repetitions)]
pub trait WorkingFluid {
    /// Returns information about the fluid model
    fn report(&self) -> String;
}
