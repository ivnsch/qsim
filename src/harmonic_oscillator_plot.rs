use std::f32::consts::{E, PI};

use crate::{
    plot::{self, setup_curve, Curve, CurvePDF, CurveWave},
    ui::EnergyLevel,
};
use bevy::{
    color::palettes::{css::WHITE, tailwind::GRAY_500},
    prelude::*,
};

// const H_BAR: f32 = 0.1;
const H_BAR: f32 = 1.054571817e-34;

pub fn add_plot(app: &mut App) {
    plot::add_plot(app);
    app.add_systems(Update, (setup_pdf, setup_wave));
}

fn setup_wave(
    mut commands: Commands,
    energy_level_query: Query<&EnergyLevel>,
    curve_query: Query<Entity, (With<Curve>, With<CurveWave>)>,
) {
    for e in energy_level_query.iter() {
        setup_curve(
            &mut commands,
            |x| wave(x, e, 9e-31, 10e16_f32),
            GRAY_500,
            e.0,
            &curve_query,
        );
    }
}

fn setup_pdf(
    mut commands: Commands,
    energy_level_query: Query<&EnergyLevel>,
    curve_query: Query<Entity, (With<Curve>, With<CurvePDF>)>,
) {
    for e in energy_level_query.iter() {
        setup_curve(
            &mut commands,
            |x| wave(x, e, 9e-31, 10e16_f32).powi(2),
            WHITE,
            e.0,
            &curve_query,
        );
    }
}

fn wave(x: f32, level: &EnergyLevel, mass: f32, ang_freq: f32) -> f32 {
    let normalization_constant = calculate_normalization_constant(level, mass, ang_freq);
    let e_exp = -(mass * ang_freq * x.powi(2)) / (2.0 * H_BAR);
    let e_term = E.powf(e_exp);
    let pol = hermite_polynomial(level);
    let pol_param = ((mass * ang_freq) / H_BAR).sqrt() * x;

    let res = normalization_constant * e_term * pol(pol_param);
    // normalization_constant * e_term * pol(pol_param)

    // println!("level: {}, x: {} -> res: {}", level.0, x, res);

    res
}

fn calculate_normalization_constant(level: &EnergyLevel, mass: f32, ang_freq: f32) -> f32 {
    // println!(
    //     "pars: level: {}, mass: {}, ang_freq: {}",
    //     level.0, mass, ang_freq
    // );
    let two_float = 2.0_f32;
    let level_int = level.0 as i32;
    let level_uint = level.0 as u32;

    let level_fact: u32 = (1..=level_uint).product();
    // println!("level_fact: {}", level_fact);

    let term1 = 1.0 / (two_float.powi(level_int) * level_fact as f32).sqrt();
    // println!("term1: {}", term1);
    let term2 = ((mass * ang_freq) / (PI * H_BAR)).powf(1.0 / 4.0);
    // println!("term2: {}", term2);

    term1 * term2
}

// TODO calculate dynamically
fn hermite_polynomial(level: &EnergyLevel) -> impl Fn(f32) -> f32 {
    match level.0 {
        0 => |_| 1.0,
        1 => |y| 2.0 * y,
        2 => |y: f32| (4.0 * y).powi(2) - 2.0,
        3 => |y: f32| (8.0 * y).powi(3) - (12.0 * y),
        _ => panic!("TODO generate polynomials dynamically"),
    }
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;
    use bevy::math::Vec2;

    use crate::{plot::generate_points, ui::EnergyLevel};

    use super::{calculate_normalization_constant, wave};

    #[test]
    fn generates_correct_domain_points() {
        let domain_points = generate_points(-2e-10, 2e-10, 1e-10, |x| x * 2.0);

        assert_eq!(5, domain_points.len());
        assert_eq!(Vec2::new(-2e-10, -4e-10), domain_points[0]);
        assert_eq!(Vec2::new(-1e-10, -2e-10), domain_points[1]);
        assert_eq!(Vec2::new(0.0, 0.0), domain_points[2]);
        assert_eq!(Vec2::new(1e-10, 2e-10), domain_points[3]);
        assert_eq!(Vec2::new(2e-10, 4e-10), domain_points[4]);
    }

    #[test]
    fn waves_for_e_0_x_0_are_correct() {
        let mass = 1.0;
        let ang_freq = 1.0;

        let level = EnergyLevel(0);
        let x = 0.0;

        let n = calculate_normalization_constant(&level, mass, ang_freq);

        let psi = wave(x, &level, mass, ang_freq);
        // todo separate fn
        let pd = psi.powi(2);

        assert_eq!(234392381.5, n);
        // psi is the normalization constant: x = 0 makes the rest of the equation 1
        assert_relative_eq!(n, psi);
        assert_relative_eq!(5.49397885e16, pd, epsilon = 0.00000000000001);
    }

    #[test]
    fn waves_for_e_0_x_2_are_correct() {
        let mass = 1.0;
        let ang_freq = 1.0;

        let level = EnergyLevel(0);
        let x = 2.0;

        let n = calculate_normalization_constant(&level, mass, ang_freq);

        let psi = wave(x, &level, mass, ang_freq);
        // todo separate fn
        let pdf = psi.powi(2);

        assert_eq!(234392381.5, n);
        assert_relative_eq!(0.0, psi);
        assert_relative_eq!(0.0, pdf);
    }

    #[test]
    fn waves_for_e_0_x_0_realistic_pars_are_correct() {
        let mass = 9.11e-31;
        let ang_freq = 1e16_f32;

        let level = EnergyLevel(0);
        let x = 0.0;

        let n = calculate_normalization_constant(&level, mass, ang_freq);

        let psi = wave(x, &level, mass, ang_freq);
        // todo separate fn
        let pd = psi.powi(2);

        assert_eq!(72414.09141, n);
        assert_relative_eq!(n, psi);
        assert_relative_eq!(5.24380063e9, pd, epsilon = 0.00000000000001);
    }

    #[test]
    fn waves_for_e_0_x_nonzero_realistic_pars_are_correct() {
        let mass = 9.11e-31;
        let ang_freq = 1e16_f32;

        let level = EnergyLevel(0);
        let x = -1e-10;

        let n = calculate_normalization_constant(&level, mass, ang_freq);

        let psi = wave(x, &level, mass, ang_freq);
        // todo separate fn
        let pd = psi.powi(2);

        assert_eq!(72414.09141, n);
        assert_relative_eq!(47015.25181, psi);
        assert_relative_eq!(2.2104339e9, pd, epsilon = 0.00000000000001);
    }
}
