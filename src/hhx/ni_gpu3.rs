use crate::ParasiticPower;

pub struct NuclearIsomerGPU3 {}

impl super::HotHeatExchanger for NuclearIsomerGPU3 {
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
