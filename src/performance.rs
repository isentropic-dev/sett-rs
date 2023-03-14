use na::DVector;

use crate::{fluid::Fluid, Engine};

#[allow(non_snake_case)]
/// Pressures in the expansion and compressions spaces, accounting for pressure
/// drops in the HXs.
pub(crate) struct PressuresWithDrops {
    pub P_c: DVector<f64>,
    pub P_e: DVector<f64>,
}

impl<T: Fluid> From<&Engine<T>> for PressuresWithDrops {
    fn from(engine: &Engine<T>) -> Self {
        let density_func = |temp: f64, pres: f64| -> f64 { engine.state.fluid.dens(temp, pres) };

        let cold_hx = Self::calculate_hx_pressure_drop(
            (&engine.values.m_dot_ck, &engine.values.m_dot_kr),
            engine.state.temp.chx,
            &engine.values.P,
            engine
                .components
                .chx
                .hydraulic_resistance(&engine.state.chx()),
            density_func,
        );
        let hot_hx = Self::calculate_hx_pressure_drop(
            (&engine.values.m_dot_rl, &engine.values.m_dot_le),
            engine.state.temp.hhx,
            &engine.values.P,
            engine
                .components
                .hhx
                .hydraulic_resistance(&engine.state.hhx()),
            density_func,
        );
        let regen = Self::calculate_hx_pressure_drop(
            (&engine.values.m_dot_kr, &engine.values.m_dot_rl),
            engine.state.temp.regen.avg,
            &engine.values.P,
            engine
                .components
                .regen
                .hydraulic_resistance(&engine.state.regen()),
            density_func,
        );
        let total = &cold_hx + &hot_hx + &regen;

        let pressure = DVector::from_row_slice(&engine.values.P);

        Self {
            P_c: &pressure + 0.5 * &total,
            P_e: &pressure - 0.5 * &total,
        }
    }
}

impl PressuresWithDrops {
    /// Calculate the pressure drop in an HX.
    fn calculate_hx_pressure_drop<F>(
        m_dots: (&[f64], &[f64]),
        temp: f64,
        pressures: &[f64],
        hydraulic_resistance: f64,
        density_func: F,
    ) -> DVector<f64>
    where
        F: Fn(f64, f64) -> f64,
    {
        let m_dot_avg =
            0.5 * (DVector::from_row_slice(m_dots.0) + DVector::from_row_slice(m_dots.1));
        let pres_vec = DVector::from_row_slice(pressures);
        let density = pres_vec.map(|pressure| density_func(temp, pressure));
        let volumetric_flow_rate = m_dot_avg.component_div(&density);

        hydraulic_resistance * volumetric_flow_rate
    }
}

#[cfg(test)]
mod test {
    use na::DVector;

    use crate::performance::PressuresWithDrops;

    #[test]
    fn calculating_hx_pressure_drop() {
        let m_dots = (&vec![10.; 2][..], &vec![20.; 2][..]);
        let temp = 200.;
        let pressures = &[100.; 2];
        let hydraulic_resistance = 2.;
        let density_func = |temp: f64, pres: f64| -> f64 { temp + pres };

        let result = PressuresWithDrops::calculate_hx_pressure_drop(
            m_dots,
            temp,
            pressures,
            hydraulic_resistance,
            density_func,
        );

        assert_eq!(result, DVector::from_element(2, 0.1));
    }
}
