use crate::ParasiticPower;

pub struct GPU3 {}

impl super::HotHeatExchanger for GPU3 {
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
