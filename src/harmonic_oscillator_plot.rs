/// basic quantum harmonic oscillator plot
/// it plots Ψ(x) and PDF(x) for a given energy level, selected via the UI
/// we use the solved equations for Ψ and PDF
use crate::{
    plot::{
        generate_points, setup_curve, setup_plot_ticks, Curve, CurvePDF, CurveWave, PlotSettings,
        TickSettings,
    },
    ui::{EnergyLevel, PotentialModelInput},
};
use bevy::{
    color::palettes::{css::WHITE, tailwind::GRAY_500},
    prelude::*,
};
use std::f32::consts::{E, PI};
use uom::si::{
    f32::{Frequency, Length, Mass},
    frequency::hertz,
    length::meter,
    mass::kilogram,
};

const H_BAR: f32 = 1.054571817e-34;

/// make settings specific to this plot type
/// needed for bevy's resources specifics
#[derive(Resource)]
pub struct HarmonicOscillatorPlotSettings(pub PlotSettings);

/// adds this plot to the app
pub fn add_plot(app: &mut App) {
    app.add_systems(
        Update,
        (setup_pdf, setup_psi, setup_ticks).run_if(is_model_selected),
    )
    .insert_resource(HarmonicOscillatorPlotSettings(PlotSettings {
        domain_range_start: -2e-10,
        domain_range_end: 2e-10,
        screen_scale_x: 1e10,
        screen_scale_y_psi: 1.0 / 72414.0,
        screen_scale_y_pdf: 1.0 / 8000000000.0,
        ticks: TickSettings { step: 1e-10 },
    }));
}

/// condition to add this plot
fn is_model_selected(mode: Res<PotentialModelInput>) -> bool {
    match *mode {
        PotentialModelInput::HarmonicOscillator => true,
        _ => false,
    }
}

/// adds Ψ screen curve to bevy
fn setup_psi(
    mut commands: Commands,
    energy_level_query: Query<&EnergyLevel>,
    curve_query: Query<Entity, (With<Curve>, With<CurveWave>)>,
    settings: Res<HarmonicOscillatorPlotSettings>,
) {
    let mass = Mass::new::<kilogram>(9e-31);
    let ang_freq = Frequency::new::<hertz>(10e16_f32);
    for e in energy_level_query.iter() {
        let points = generate_psi_points(|x| psi(x, e, mass, ang_freq), &settings.0.clone());
        setup_curve(&mut commands, WHITE, e.0, &curve_query, points);
    }
}

/// adds PDF screen curve to bevy
fn setup_pdf(
    mut commands: Commands,
    energy_level_query: Query<&EnergyLevel>,
    curve_query: Query<Entity, (With<Curve>, With<CurvePDF>)>,
    settings: Res<HarmonicOscillatorPlotSettings>,
) {
    let mass = Mass::new::<kilogram>(9e-31);
    let ang_freq = Frequency::new::<hertz>(10e16_f32);
    for e in energy_level_query.iter() {
        let points = generate_pdf_points(|x| pdf(x, e, mass, ang_freq), &settings.0.clone());
        setup_curve(&mut commands, GRAY_500, e.0, &curve_query, points);
    }
}

/// Ψ_n(x), see https://en.wikipedia.org/wiki/Quantum_harmonic_oscillator#Hamiltonian_and_energy_eigenstates
fn psi(x: Length, level: &EnergyLevel, mass: Mass, ang_freq: Frequency) -> f32 {
    let normalization_constant = calculate_normalization_constant(level, mass, ang_freq);

    let sub_term = (mass * ang_freq) / H_BAR;
    let sub_term_value = sub_term.value;
    let x_value = x.value;

    let e_exp = -sub_term_value * x_value.powi(2) / 2.0;
    let e_term = E.powf(e_exp);

    let pol = hermite_polynomial(level);

    let pol_param = sub_term_value.sqrt() * x_value;

    let res = normalization_constant * e_term * pol(pol_param);

    res
}

/// PDF for Ψ_n(x)
fn pdf(x: Length, level: &EnergyLevel, mass: Mass, ang_freq: Frequency) -> f32 {
    let psi = psi(x, level, mass, ang_freq);
    psi.powi(2)
}

/// step in Ψ calculation, for better readability
fn calculate_normalization_constant(level: &EnergyLevel, mass: Mass, ang_freq: Frequency) -> f32 {
    let two_float = 2.0_f32;
    let level_int = level.0 as i32;
    let level_uint = level.0 as u32;

    let level_fact: u32 = (1..=level_uint).product();

    let term1 = 1.0 / (two_float.powi(level_int) * level_fact as f32).sqrt();

    let sub_term = (mass * ang_freq) / H_BAR;
    let sub_term_value = sub_term.value;

    let term2 = (sub_term_value / PI).powf(1.0 / 4.0);

    term1 * term2
}

/// generates Ψ screen points
fn generate_psi_points<F>(function: F, settings: &PlotSettings) -> Vec<Vec2>
where
    // for now assuming the dimension to be spatial
    F: Fn(Length) -> f32,
{
    // scaled down y by ~max value so it fits in graph
    // TODO generic mapping to screen coords
    generate_psi_or_pdf_points(function, settings.screen_scale_y_psi, settings)
}

/// generates Ψ pdf points
fn generate_pdf_points<F>(function: F, settings: &PlotSettings) -> Vec<Vec2>
where
    // for now assuming the dimension to be spatial
    F: Fn(Length) -> f32,
{
    // scaled dowwn y by eye to plot together with psi
    // exact height unimportant
    generate_psi_or_pdf_points(function, settings.screen_scale_y_pdf, settings)
}

/// generates screen points
fn generate_psi_or_pdf_points<F>(function: F, scale_y: f32, settings: &PlotSettings) -> Vec<Vec2>
where
    // for now assuming the dimension to be spatial
    F: Fn(Length) -> f32,
{
    let domain_points = generate_points(
        settings.domain_range_start,
        settings.domain_range_end,
        1e-12,
        |x| function(Length::new::<meter>(x)),
    );
    let scaled_points: Vec<Vec2> = domain_points
        .into_iter()
        .map(|p| Vec2::new(p.x * settings.screen_scale_x, p.y * scale_y)) // wave
        .collect();

    scaled_points
}

/// generates the hermite polynomial for a given energy level
/// ideally it should be done dynamically (allowing for principally infinite levels),
/// but not entirely trivial in rust (TODO)
/// for now hardcoded the polynomials for the 10 first energy levels.
fn hermite_polynomial(level: &EnergyLevel) -> impl Fn(f32) -> f32 {
    match level.0 {
        0 => |_| 1.0,
        1 => |y| 2.0 * y,
        2 => |y: f32| 4.0 * y.powi(2) - 2.0,
        3 => |y: f32| 8.0 * y.powi(3) - 12.0 * y,
        4 => |y: f32| 16.0 * y.powi(4) - 48.0 * y.powi(2) + 12.0,
        5 => |y: f32| 32.0 * y.powi(5) - 160.0 * y.powi(3) + 120.0 * y,
        6 => |y: f32| 64.0 * y.powi(6) - 480.0 * y.powi(4) + 720.0 * y.powi(2) - 120.0,
        7 => |y: f32| 128.0 * y.powi(7) - 1344.0 * y.powi(5) + 3360.0 * y.powi(3) - 1680.0 * y,
        8 => |y: f32| {
            256.0 * y.powi(8) - 3584.0 * y.powi(6) + 13440.0 * y.powi(4) - 13440.0 * y.powi(2)
                + 1680.0
        },
        9 => |y: f32| {
            512.0 * y.powi(9) - 9216.0 * y.powi(7) + 48384.0 * y.powi(5) - 80640.0 * y.powi(3)
                + 30240.0 * y
        },
        10 => |y: f32| {
            1024.0 * y.powi(10) - 23040.0 * y.powi(8) + 161280.0 * y.powi(6) - 403200.0 * y.powi(4)
                + 302400.0 * y.powi(2)
                + 30240.0
        },
        // leniently using panic!, implementation detail, don't want to add noise downstream
        _ => panic!("TODO polynomials not supported for n > 10"),
    }
}

fn setup_ticks(mut gizmos: Gizmos, settings: Res<HarmonicOscillatorPlotSettings>) {
    setup_plot_ticks(&mut gizmos, settings.0.clone())
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;
    use bevy::math::Vec2;
    use uom::si::{
        f32::{Frequency, Length, Mass},
        frequency::hertz,
        length::meter,
        mass::{gram, kilogram},
    };

    use crate::{harmonic_oscillator_plot::pdf, plot::generate_points, ui::EnergyLevel};

    use super::{calculate_normalization_constant, psi};

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
        let mass = Mass::new::<kilogram>(1.0);
        let ang_freq = Frequency::new::<hertz>(1.0);

        let level = EnergyLevel(0);
        let x = Length::new::<meter>(0.0);

        let n = calculate_normalization_constant(&level, mass, ang_freq);

        let psi = psi(x, &level, mass, ang_freq);
        let pd = pdf(x, &level, mass, ang_freq);

        assert_eq!(234392381.5, n);
        // psi = normalization constant: x = 0 makes the rest of the equation 1
        assert_relative_eq!(n, psi);
        assert_relative_eq!(5.49397885e16, pd, epsilon = 0.00000000000001);
    }

    #[test]
    fn waves_for_e_0_x_2_are_correct() {
        let mass = Mass::new::<kilogram>(1.0);
        let ang_freq = Frequency::new::<hertz>(1.0);

        let level = EnergyLevel(0);
        let x = Length::new::<meter>(2.0);

        let n = calculate_normalization_constant(&level, mass, ang_freq);

        let psi = psi(x, &level, mass, ang_freq);
        let pd = pdf(x, &level, mass, ang_freq);

        assert_eq!(234392381.5, n);
        assert_relative_eq!(0.0, psi);
        assert_relative_eq!(0.0, pd);
    }

    #[test]
    fn waves_for_e_0_x_0_realistic_pars_are_correct() {
        let mass = Mass::new::<kilogram>(9.11e-31);
        let ang_freq = Frequency::new::<hertz>(1e16_f32);

        let level = EnergyLevel(0);
        let x = Length::new::<meter>(0.0);

        let n = calculate_normalization_constant(&level, mass, ang_freq);

        let psi = psi(x, &level, mass, ang_freq);
        let pd = pdf(x, &level, mass, ang_freq);

        assert_eq!(72414.09141, n);
        assert_relative_eq!(n, psi);
        assert_relative_eq!(5.24380063e9, pd, epsilon = 0.00000000000001);
    }

    #[test]
    fn waves_for_e_0_x_nonzero_realistic_pars_are_correct() {
        let mass = Mass::new::<kilogram>(9.11e-31);
        let ang_freq = Frequency::new::<hertz>(1e16_f32);

        let level = EnergyLevel(0);
        let x = Length::new::<meter>(-1e-10);

        let n = calculate_normalization_constant(&level, mass, ang_freq);

        let psi = psi(x, &level, mass, ang_freq);
        let pd = pdf(x, &level, mass, ang_freq);

        assert_eq!(72414.09141, n);
        assert_relative_eq!(47015.25181, psi);
        assert_relative_eq!(2.2104339e9, pd, epsilon = 0.00000000000001);
    }

    /// just double checking `value` property
    /// it uses [SI base units](https://en.wikipedia.org/wiki/SI_base_unit) (hardcoded)
    #[test]
    fn uom_clarification() {
        let mass1 = Mass::new::<kilogram>(1.0);
        assert_relative_eq!(1.0, mass1.value); // base unit (kg)
        assert_relative_eq!(1.0, mass1.get::<kilogram>());
        assert_relative_eq!(1000.0, mass1.get::<gram>());

        let mass2 = Mass::new::<gram>(1.0);
        assert_relative_eq!(0.001, mass2.value); // base unit (kg)
        assert_relative_eq!(0.001, mass2.get::<kilogram>());
        assert_relative_eq!(1.0, mass2.get::<gram>());
    }
}
