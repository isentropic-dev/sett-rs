use crate::ParasiticPower;

pub struct FixedConductance {}

impl super::HotHeatExchanger for FixedConductance {
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
