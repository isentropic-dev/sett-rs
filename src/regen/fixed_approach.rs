use crate::ParasiticPower;

pub struct FixedApproach {}

impl super::Regenerator for FixedApproach {
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
