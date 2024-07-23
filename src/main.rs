mod plot;

use bevy::app::App;
use plot::add_plot;

fn main() {
    let app = &mut App::new();
    add_plot(app);
    app.run();
}
