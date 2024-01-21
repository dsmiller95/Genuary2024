mod boids_sim;

use bevy::prelude::*;
use boids_sim::app::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BoidsSimPlugin))
        .add_systems(Startup, size_window)
        .run();
}

fn size_window(mut query: Query<&mut Window>) {
    let mut window = query.single_mut();
    window.resolution.set_physical_resolution(1920, 1080);
    window.position.center(MonitorSelection::Current);
    window.title = "Boids".to_string();
}