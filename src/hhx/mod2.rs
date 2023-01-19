use crate::ParasiticPower;

pub struct Mod2 {}

impl super::HotHeatExchanger for Mod2 {
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
