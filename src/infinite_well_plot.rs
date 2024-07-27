use crate::{
    plot::{self, generate_points, setup_curve, Curve, CurvePDF, CurveWave},
    ui::EnergyLevel,
};
use bevy::{
    color::palettes::{css::WHITE, tailwind::GRAY_500},
    prelude::*,
};
use std::f32::consts::PI;

pub fn add_plot(app: &mut App) {
    plot::add_plot(app);
    app.add_systems(Update, (setup_pdf, setup_psi));
}

fn setup_psi(
    mut commands: Commands,
    energy_level_query: Query<&EnergyLevel>,
    curve_query: Query<Entity, (With<Curve>, With<CurveWave>)>,
) {
    for e in energy_level_query.iter() {
        let points = generate_psi_points(|x| psi(x, e));
        setup_curve(&mut commands, GRAY_500, e.0, &curve_query, points);
    }
}

fn setup_pdf(
    mut commands: Commands,
    energy_level_query: Query<&EnergyLevel>,
    curve_query: Query<Entity, (With<Curve>, With<CurvePDF>)>,
) {
    for e in energy_level_query.iter() {
        let points = generate_pdf_points(|x| pdf(x, e));
        setup_curve(&mut commands, WHITE, e.0, &curve_query, points);
    }
}

fn generate_psi_points<F>(function: F) -> Vec<Vec2>
where
    F: Fn(f32) -> f32,
{
    generate_psi_or_pdf_points(function, 1.0)
}

fn generate_pdf_points<F>(function: F) -> Vec<Vec2>
where
    F: Fn(f32) -> f32,
{
    generate_psi_or_pdf_points(function, 1.0)
}

fn generate_psi_or_pdf_points<F>(function: F, scale_y: f32) -> Vec<Vec2>
where
    F: Fn(f32) -> f32,
{
    let domain_points = generate_points(-10.0, 10.0, 0.02, function);
    let scaled_points: Vec<Vec2> = domain_points
        .into_iter()
        .map(|p| Vec2::new(p.x * 1e10, p.y * scale_y)) // wave
        .collect();

    scaled_points
}

/// Ψ_n(x)
fn psi(x: f32, level: &EnergyLevel) -> f32 {
    let l: f32 = 2.0;
    (2.0 / l).sqrt() * ((level.0 as f32 * PI * x) / l).sin()
}

/// PDF for Ψ_n(x)
fn pdf(x: f32, level: &EnergyLevel) -> f32 {
    let psi = psi(x, level);
    psi.powi(2)
}
