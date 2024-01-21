mod boids_sim;

use bevy::prelude::*;
use boids_sim::app::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BoidsSimPlugin))
        .run();
}
