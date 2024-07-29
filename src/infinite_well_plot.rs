use crate::{
    plot::{generate_points, setup_curve, Curve, CurvePDF, CurveWave, PlotSettings},
    ui::{EnergyLevel, PotentialModel, PotentialModelInput},
};
use bevy::{
    color::palettes::{
        css::{GRAY, GREEN, WHITE},
        tailwind::GRAY_500,
    },
    prelude::*,
};
use std::f32::consts::PI;

pub fn add_plot(app: &mut App) {
    app.add_systems(
        Update,
        (
            setup_pdf,
            setup_psi,
            setup_ticks,
            setup_vertical_dashed_line,
        ),
    )
    .insert_resource(PlotSettings::default());
}

fn setup_psi(
    mut commands: Commands,
    energy_level_query: Query<&EnergyLevel>,
    curve_query: Query<Entity, (With<Curve>, With<CurveWave>)>,
    model: Query<&PotentialModel>,
) {
    for m in model.iter() {
        if m.0 == PotentialModelInput::InfiniteWell {
            for e in energy_level_query.iter() {
                let points = generate_scaled_points(|x| psi(x, e));
                setup_curve(&mut commands, GRAY_500, e.0, &curve_query, points);
            }
        }
    }
}

fn setup_pdf(
    mut commands: Commands,
    energy_level_query: Query<&EnergyLevel>,
    curve_query: Query<Entity, (With<Curve>, With<CurvePDF>)>,
    model: Query<&PotentialModel>,
) {
    for m in model.iter() {
        if m.0 == PotentialModelInput::InfiniteWell {
            for e in energy_level_query.iter() {
                let points = generate_scaled_points(|x| pdf(x, e));
                setup_curve(&mut commands, WHITE, e.0, &curve_query, points);
            }
        }
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

fn setup_ticks(mut gizmos: Gizmos, model: Query<&PotentialModel>, settings: Res<PlotSettings>) {
    for m in model.iter() {
        if m.0 == PotentialModelInput::InfiniteWell {
            let domain_points = generate_points(
                settings.domain_range_start,
                settings.domain_range_end,
                settings.ticks_step,
                |x| x,
            );
            let line_height = 0.1;
            let half_line_height = line_height / 2.0;
            for point in domain_points {
                let x = point.x * settings.screen_scale_x;
                gizmos.line_2d(
                    Vec2 {
                        x,
                        y: -half_line_height,
                    },
                    Vec2 {
                        x,
                        y: half_line_height,
                    },
                    GREEN,
                );
            }
        }
    }
}

fn setup_vertical_dashed_line(mut gizmos: Gizmos, model: Query<&PotentialModel>) {
    for m in model.iter() {
        if m.0 == PotentialModelInput::InfiniteWell {
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
}
