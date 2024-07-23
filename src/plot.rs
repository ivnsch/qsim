use bevy::{color::palettes::css::WHITE, math::vec3, prelude::*};

pub fn add_plot(app: &mut App) {
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_light, setup))
        .add_systems(Update, draw_curve);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 6., 12.).looking_at(Vec3::new(0., 3., 0.), Vec3::Y),
        ..default()
    });
}

fn setup_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
}

fn setup(mut commands: Commands) {
    let points = [[
        vec3(-6., 2., 0.),
        vec3(12., 8., 0.),
        vec3(-12., 8., 0.),
        vec3(6., 2., 0.),
    ]];

    let bezier = CubicBezier::new(points).to_curve();
    commands.spawn(Curve(bezier));
}

#[derive(Component)]
struct Curve(CubicCurve<Vec3>);

fn draw_curve(mut query: Query<&Curve>, mut gizmos: Gizmos) {
    for cubic_curve in &mut query {
        // Draw the curve
        gizmos.linestrip(cubic_curve.0.iter_positions(50), WHITE);
    }
}
