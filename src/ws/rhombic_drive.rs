use super::{CompVolume, ExpVolume, Parasitics, State, ThermalResistance, WorkingSpaces};

pub struct RhombicDrive {}

impl WorkingSpaces for RhombicDrive {
    fn frequency(&self, _state: &State) -> f64 {
        todo!()
    }

    fn volumes(&self, _state: &State) -> Box<(dyn Fn(f64) -> (CompVolume, ExpVolume))> {
        todo!()
    }

    fn thermal_resistance(&self, _state: &State) -> ThermalResistance {
        todo!()
    }

    fn parasitics(&self, _state: &State) -> Parasitics {
        todo!()
    }
}
