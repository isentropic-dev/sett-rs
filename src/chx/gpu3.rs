use crate::ParasiticPower;

pub struct GPU3 {}

impl super::ColdHeatExchanger for GPU3 {
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
