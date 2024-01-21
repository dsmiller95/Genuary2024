use crate::boids_sim::boid_bundle::BoidBundle;
use crate::boids_sim::components::*;
use crate::boids_sim::update_systems::*;
use super::prelude::*;

pub struct BoidsSimPlugin;
impl Plugin for BoidsSimPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PrintTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_systems(Startup, (add_boids, add_rendering))
            .add_systems(Update, (
                (add_velocity_to_position, set_pos_vel_to_transform).chain(),
                print_positions)
            );
    }
}

fn add_rendering(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn add_boids(mut commands: Commands) {
    let mut rng = SmallRng::from_entropy();
    for _ in 0..BOID_N {
        commands.spawn(BoidBundle::new(&mut rng, SPACE_SIZE as f32, SPAWN_VEL));
    }
    println!("Added {} boids", BOID_N);
}