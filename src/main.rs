mod infinite_well_plot;
mod plot;
mod ui;

use bevy::app::App;

fn main() {
    let app = &mut App::new();
    infinite_well_plot::add_plot(app);
    app.run();
}
