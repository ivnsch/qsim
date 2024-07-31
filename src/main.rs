mod camera_controller;
mod harmonic_oscillator_plot;
mod infinite_well_plot;
mod plot;
mod ui;

use bevy::app::App;

fn main() {
    let app = &mut App::new();
    plot::add_plot(app);
    infinite_well_plot::add_plot(app);
    harmonic_oscillator_plot::add_plot(app);
    app.run();
}
