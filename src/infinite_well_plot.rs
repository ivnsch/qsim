use crate::{
    plot::{self, setup_curve, Curve, CurvePDF, CurveWave},
    ui::EnergyLevel,
};
use bevy::{
    color::palettes::{css::WHITE, tailwind::GRAY_500},
    prelude::*,
};
use std::f32::consts::PI;

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
        setup_curve(&mut commands, |x| wave(x, e), GRAY_500, e.0, &curve_query);
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
            |x| wave(x, e).powi(2),
            WHITE,
            e.0,
            &curve_query,
        );
    }
}

fn wave(x: f32, level: &EnergyLevel) -> f32 {
    let l: f32 = 2.0;
    (2.0 / l).sqrt() * ((level.0 as f32 * PI * x) / l).sin()
}
