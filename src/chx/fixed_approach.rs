use crate::ParasiticPower;

pub struct FixedApproach {}

impl super::ColdHeatExchanger for FixedApproach {
    fn approach(&self) -> f64 {
        todo!()
    }

    fn pressure_drop(&self) -> &[f64] {
        todo!()
    }

    fn parasitics(&self) -> ParasiticPower {
        todo!()
    }
}
