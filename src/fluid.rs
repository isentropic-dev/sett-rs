mod fit;
mod ideal_gas;
mod refprop;

// Export all available fluid models
pub use ideal_gas::IdealGas;

pub trait Fluid {}
