mod seed_sim;

use bevy::app::{App, Startup};
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::prelude::{MonitorSelection, Query, Window};
use crate::seed_sim::app::PlantSimPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlantSimPlugin))
        .add_systems(Startup, size_window)
        .run();
}

fn size_window(mut query: Query<&mut Window>) {
    let mut window = query.single_mut();
    window.resolution.set_physical_resolution(1920, 1080);
    window.position.center(MonitorSelection::Current);
    window.title = "Boids".to_string();
}