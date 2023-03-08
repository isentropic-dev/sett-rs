use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub enum FrictionFactorCorrelation {
    GedeonWood,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum JFactorCorrelation {
    GedeonWood,
}
