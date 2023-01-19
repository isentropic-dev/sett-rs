mod fit;
mod ideal_gas;
mod refprop;

pub use ideal_gas::IdealGas;

pub trait WorkingFluid: std::fmt::Display {}
