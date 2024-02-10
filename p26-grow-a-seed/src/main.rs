mod seed_sim;

use bevy::app::{App, Startup};
use bevy::core::TaskPoolThreadAssignmentPolicy;
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::tasks::available_parallelism;
use crate::seed_sim::app::PlantSimPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(TaskPoolPlugin {
            task_pool_options: TaskPoolOptions {
                compute: TaskPoolThreadAssignmentPolicy {
                    // set the minimum # of compute threads
                    // to the total number of available threads
                    min_threads: 1,
                    max_threads: available_parallelism().min(4), // limit the max # of compute threads to 4
                    percent: 1.0, // this value is irrelevant in this case
                },
                // keep the defaults for everything else
                ..default()
            }
        }))
        .add_plugins((PlantSimPlugin))
        .add_systems(Startup, size_window)
        .run();
}

fn size_window(mut query: Query<&mut Window>) {
    let mut window = query.single_mut();
    window.resolution.set_physical_resolution(1920, 1080);
    window.position.center(MonitorSelection::Current);
    window.title = "Boids".to_string();
}