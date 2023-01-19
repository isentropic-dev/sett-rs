use crate::ParasiticPower;

pub struct NuclearIsomerMod2 {}

impl super::HotHeatExchanger for NuclearIsomerMod2 {
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
