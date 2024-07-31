use crate::{
    plot::{
        generate_points, setup_curve, setup_plot_ticks, Curve, CurvePDF, CurveWave, PlotSettings,
    },
    ui::{EnergyLevel, PotentialModelInput},
};
use bevy::{
    color::palettes::{
        css::{GRAY, WHITE},
        tailwind::GRAY_500,
    },
    prelude::*,
};
use std::f32::consts::PI;

#[derive(Resource)]
pub struct InfiniteWellPlotSettings(PlotSettings);

pub fn add_plot(app: &mut App) {
    app.add_systems(
        Update,
        (
            setup_pdf,
            setup_psi,
            setup_ticks,
            setup_vertical_dashed_line,
        )
            .run_if(is_model_selected),
    )
    .insert_resource(InfiniteWellPlotSettings(PlotSettings::default()));
}

fn is_model_selected(mode: Res<PotentialModelInput>) -> bool {
    match *mode {
        PotentialModelInput::InfiniteWell => true,
        _ => false,
    }
}

fn setup_psi(
    mut commands: Commands,
    energy_level_query: Query<&EnergyLevel>,
    curve_query: Query<Entity, (With<Curve>, With<CurveWave>)>,
) {
    for e in energy_level_query.iter() {
        let points = generate_scaled_points(|x| psi(x, e));
        setup_curve(&mut commands, WHITE, e.0, &curve_query, points);
    }
}

fn setup_pdf(
    mut commands: Commands,
    energy_level_query: Query<&EnergyLevel>,
    curve_query: Query<Entity, (With<Curve>, With<CurvePDF>)>,
) {
    for e in energy_level_query.iter() {
        let points = generate_scaled_points(|x| pdf(x, e));
        setup_curve(&mut commands, GRAY_500, e.0, &curve_query, points);
    }
}

fn generate_scaled_points<F>(function: F) -> Vec<Vec2>
where
    F: Fn(f32) -> f32,
{
    let domain_points = generate_points(-10.0, 10.0, 0.02, function);
    // for now no scaling needed, domain parameters happen to match screen dimensions
    domain_points
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

fn setup_vertical_dashed_line(mut gizmos: Gizmos, model: Res<PotentialModelInput>) {
    if *model == PotentialModelInput::InfiniteWell {
        let x = 2.0;
        // for now hardcoded
        let mut y_start = -10_f32;
        while y_start < 10_f32 {
            gizmos.line_2d(
                Vec2 { x, y: y_start },
                Vec2 {
                    x,
                    y: y_start + 0.06,
                },
                GRAY,
            );

            y_start += 0.1;
        }
    }
}

fn setup_ticks(mut gizmos: Gizmos, settings: Res<InfiniteWellPlotSettings>) {
    setup_plot_ticks(&mut gizmos, settings.0.clone())
}
