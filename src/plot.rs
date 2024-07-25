use std::f32::consts::PI;

use bevy::{color::palettes::css::WHITE, prelude::*};

pub fn add_plot(app: &mut App) {
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_light))
        .add_systems(Startup, setup)
        .add_systems(Update, draw_curve);
}

fn setup(mut commands: Commands) {
    let l: f32 = 1.0;
    let n: f32 = 1.0;
    let domain_points = generate_points(-10, 10, 0.02, |x| {
        (2.0 / l).sqrt() * ((n * PI * x) / l).sin()
    });

    let bezier_points = generate_path(&domain_points, 0.3, 0.3);
    let bezier = CubicBezier::new(bezier_points).to_curve();
    commands.spawn(Curve(bezier));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.02,
            ..default()
        },
        ..default()
    });
}

fn setup_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
}

// https://github.com/ivnsch/SwiftCharts/blob/c354c1945bb35a1f01b665b22474f6db28cba4a2/SwiftCharts/Views/CubicLinePathGenerator
fn generate_path(points: &[Vec2], tension1: f32, tension2: f32) -> Vec<[Vec2; 4]> {
    let mut path = vec![];

    if points.is_empty() {
        return path;
    }

    let mut p0: Vec2;
    let mut p1: Vec2;
    let mut p2: Vec2;
    let mut p3: Vec2;
    let mut tension_bezier1: f32;
    let mut tension_bezier2: f32;

    let mut previous_point1 = Vec2::new(0.0, 0.0);

    for i in 0..(points.len() - 1) {
        p1 = points[i];
        p2 = points[i + 1];

        tension_bezier1 = tension1;
        tension_bezier2 = tension2;

        if i > 0 {
            p0 = previous_point1;

            if (p2.y - p1.y) == (p2.y - p0.y) {
                tension_bezier1 = 0.0;
            }
        } else {
            tension_bezier1 = 0.0;
            p0 = p1;
        }

        if i < points.len() - 2 {
            p3 = points[i + 2];
            if (p3.y - p2.y) == (p2.y - p1.y) {
                tension_bezier2 = 0.0;
            }
        } else {
            p3 = p2;
            tension_bezier2 = 0.0;
        }

        let control_point1 = Vec2::new(
            p1.x + (p2.x - p1.x) / 3.0,
            p1.y - (p1.y - p2.y) / 3.0 - (p0.y - p1.y) * tension_bezier1,
        );

        let control_point2 = Vec2::new(
            p1.x + 2.0 * (p2.x - p1.x) / 3.0,
            p1.y - 2.0 * (p1.y - p2.y) / 3.0 + (p2.y - p3.y) * tension_bezier2,
        );

        // println!(
        //     "generated control points: {}, {}",
        //     control_point1, control_point2
        // );

        path.push([p0, control_point1, control_point2, p2]);

        previous_point1 = p2;
    }

    path
}

#[derive(Component)]
struct Curve(CubicCurve<Vec2>);

fn draw_curve(mut query: Query<&Curve>, mut gizmos: Gizmos) {
    for cubic_curve in &mut query {
        // Draw the curve
        gizmos.linestrip_2d(cubic_curve.0.iter_positions(500), WHITE);
    }
}

fn generate_points<F>(range_start: i32, range_end: i32, step: f32, function: F) -> Vec<Vec2>
where
    F: Fn(f32) -> f32,
{
    // let scaling = 20.0;
    let scaling = 1.0;
    let x_scaling = scaling;
    let y_scaling = scaling;

    let mut points = vec![];
    let mut value = range_start as f32;
    while value < range_end as f32 {
        let x = value;
        let y = function(x);

        points.push(Vec2::new(x, y));

        value += step;
    }

    points
}
