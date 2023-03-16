use std::f64::consts::PI;

use itertools::Itertools;
use na::DVector;

use crate::{
    fluid::Fluid,
    ws::{self, CompVolume, ExpVolume},
    Engine,
};

pub(super) struct Performance {
    pub pressures_with_drops: PressuresWithDrops,
    pub power: Powers,
    pub heat: Heats,
    pub shaft_torque: f64,
    pub efficiency: f64,
}

#[allow(non_snake_case)]
/// Pressures in the expansion and compressions spaces, accounting for pressure
/// drops in the HXs.
pub(super) struct PressuresWithDrops {
    pub P_c: DVector<f64>,
    pub P_e: DVector<f64>,
}

#[allow(non_snake_case)]
pub(super) struct Powers {
    pub indicated: f64,
    pub indicated_zero_dP: f64,
    pub shaft: f64,
    pub net: f64,
}

pub(super) struct Heats {
    pub input: f64,
    pub rejected: f64,
}

impl<T: Fluid> From<&Engine<T>> for Performance {
    fn from(engine: &Engine<T>) -> Self {
        let frequency = engine.components.ws.frequency(&engine.state.ws());

        let pressures_with_drops = PressuresWithDrops::new(engine);
        let power = Powers::new(&pressures_with_drops, engine);
        let heat = Heats::new(&power, engine);
        let shaft_torque = power.shaft / (2. * PI * frequency);
        let efficiency = power.net / heat.input;

        Self {
            pressures_with_drops,
            power,
            heat,
            shaft_torque,
            efficiency,
        }
    }
}

impl PressuresWithDrops {
    pub(super) fn new<T: Fluid>(engine: &Engine<T>) -> Self {
        let pressure = DVector::from_row_slice(&engine.values.P);
        let dens_cold_hx =
            pressure.map(|pres| engine.state.fluid.dens(engine.state.temp.chx, pres));
        let dens_hot_hx = pressure.map(|pres| engine.state.fluid.dens(engine.state.temp.hhx, pres));
        let dens_regen =
            pressure.map(|pres| engine.state.fluid.dens(engine.state.temp.regen.avg, pres));

        let cold_hx = Self::calculate_hx_pressure_drop(
            (&engine.values.m_dot_ck, &engine.values.m_dot_kr),
            &dens_cold_hx,
            engine
                .components
                .chx
                .hydraulic_resistance(&engine.state.chx()),
        );
        let hot_hx = Self::calculate_hx_pressure_drop(
            (&engine.values.m_dot_rl, &engine.values.m_dot_le),
            &dens_hot_hx,
            engine
                .components
                .hhx
                .hydraulic_resistance(&engine.state.hhx()),
        );
        let regen = Self::calculate_hx_pressure_drop(
            (&engine.values.m_dot_kr, &engine.values.m_dot_rl),
            &dens_regen,
            engine
                .components
                .regen
                .hydraulic_resistance(&engine.state.regen()),
        );
        let total = &cold_hx + &hot_hx + &regen;

        Self {
            P_c: &pressure + 0.5 * &total,
            P_e: &pressure - 0.5 * &total,
        }
    }

    /// Calculate the pressure drop in an HX.
    fn calculate_hx_pressure_drop(
        m_dots: (&[f64], &[f64]),
        density: &DVector<f64>,
        hydraulic_resistance: f64,
    ) -> DVector<f64> {
        let m_dot_avg =
            0.5 * (DVector::from_row_slice(m_dots.0) + DVector::from_row_slice(m_dots.1));
        let volumetric_flow_rate = m_dot_avg.component_div(density);

        hydraulic_resistance * volumetric_flow_rate
    }
}

impl Powers {
    #[allow(non_snake_case)]
    fn new<T: Fluid>(pressures_with_drops: &PressuresWithDrops, engine: &Engine<T>) -> Self {
        let frequency = engine.components.ws.frequency(&engine.state.ws());
        let time = DVector::from_row_slice(&engine.values.time);
        let (dVc_dt, dVe_dt) = Self::dV_dts(
            &engine.values.time,
            &*engine.components.ws,
            &engine.state.ws(),
        );

        // Calculate indicated power.
        let indicated = frequency
            * integrate(
                &time,
                &(pressures_with_drops.P_c.component_mul(&dVc_dt)
                    + pressures_with_drops.P_e.component_mul(&dVe_dt)),
            );

        // Calculate indicated power without HX pressure drops.
        let pressure = DVector::from_row_slice(&engine.values.P);
        let indicated_zero_dP =
            frequency * integrate(&time, &pressure.component_mul(&(dVc_dt + dVe_dt)));

        // Calculate shaft power.
        let ws_parasitics = &engine.components.ws.parasitics(&engine.state.ws());
        let shaft = indicated - ws_parasitics.comp.mechanical - ws_parasitics.exp.mechanical;

        // Calculate net power.
        let cold_hx_parasitics = &engine.components.chx.parasitics(&engine.state.chx());
        let hot_hx_parasitics = &engine.components.hhx.parasitics(&engine.state.hhx());
        let net = shaft - cold_hx_parasitics.mechanical - hot_hx_parasitics.mechanical;

        Self {
            indicated,
            indicated_zero_dP,
            shaft,
            net,
        }
    }

    #[allow(non_snake_case)]
    fn dV_dts(
        time: &[f64],
        ws_component: &dyn ws::WorkingSpaces,
        ws_state: &ws::State,
    ) -> (DVector<f64>, DVector<f64>) {
        let volumes_func = ws_component.volumes(ws_state);
        let (comp_volumes, exp_volumes): &(Vec<CompVolume>, Vec<ExpVolume>) =
            &time.iter().map(|t: &f64| volumes_func(*t)).unzip();
        let dVc_dt = DVector::from_vec(comp_volumes.iter().map(|cv| cv.deriv).collect());
        let dVe_dt = DVector::from_vec(exp_volumes.iter().map(|ev| ev.deriv).collect());
        (dVc_dt, dVe_dt)
    }
}

impl Heats {
    #[allow(non_snake_case)]
    fn new<T: Fluid>(power: &Powers, engine: &Engine<T>) -> Self {
        let frequency = engine.components.ws.frequency(&engine.state.ws());
        let time = DVector::from_row_slice(&engine.values.time);

        let hhx_parasitics = engine.components.hhx.parasitics(&engine.state.hhx());
        let regen_parasitics = engine.components.regen.parasitics(&engine.state.regen());
        let ws_parasitics = engine.components.ws.parasitics(&engine.state.ws());

        let ws_thermal_resistances = engine.components.ws.thermal_resistance(&engine.state.ws());

        // Heat from HHX to the expansion space .
        let T_e = &DVector::from_row_slice(&engine.values.T_e);
        let Q_dot_e = frequency
            * integrate(
                &time,
                &((T_e - DVector::from_element(T_e.nrows(), engine.state.temp.hhx))
                    / ws_thermal_resistances.exp),
            );

        // Heat input to the HHX.
        let Q_dot_l =
            frequency * integrate(&time, &DVector::from_row_slice(&engine.values.Q_dot_l));

        // Heat from compression space to CHX.
        let T_c = &DVector::from_row_slice(&engine.values.T_c);
        let Q_dot_c = frequency
            * integrate(
                &time,
                &((T_c - DVector::from_element(T_c.nrows(), engine.state.temp.chx))
                    / ws_thermal_resistances.comp),
            );

        // Heat rejected from the CHX.
        let Q_dot_k =
            frequency * integrate(&time, &DVector::from_row_slice(&engine.values.Q_dot_k));

        // Heat rejected from pressure drops.
        let Q_dot_dP = power.indicated_zero_dP - power.indicated;

        // External parasitic heat losses.
        let Q_dot_loss_external =
            hhx_parasitics.thermal + regen_parasitics.thermal + ws_parasitics.exp.thermal;

        // Internal parasitic heat losses.
        let Q_dot_loss_internal = ws_parasitics.comp.mechanical + ws_parasitics.exp.mechanical;

        Self {
            input: Q_dot_l - Q_dot_e + Q_dot_loss_external,
            rejected: Q_dot_c + Q_dot_k + Q_dot_dP + Q_dot_loss_internal + Q_dot_loss_external,
        }
    }
}

fn integrate(x: &DVector<f64>, y: &DVector<f64>) -> f64 {
    let xs = x.iter().tuple_windows();
    let ys = y.iter().tuple_windows();
    xs.zip(ys)
        .map(|((x0, x1), (y0, y1))| (y1 + y0) * (x1 - x0) * 0.5)
        .sum()
}

#[cfg(test)]
mod test {
    use na::DVector;

    use crate::performance::PressuresWithDrops;

    #[test]
    fn calculating_hx_pressure_drop() {
        let m_dots = (&vec![10.; 2][..], &vec![20.; 2][..]);
        let hydraulic_resistance = 2.;

        // Calculate density using a whack function.
        let temp = 200.;
        let pressure = DVector::from_row_slice(&[100.; 2]);
        let density = pressure.map(|pres| temp + pres);

        let result =
            PressuresWithDrops::calculate_hx_pressure_drop(m_dots, &density, hydraulic_resistance);

        assert_eq!(result, DVector::from_element(2, 0.1));
    }
}
