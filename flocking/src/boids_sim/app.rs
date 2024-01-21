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
            .insert_resource(BoidBehavior {
                space_size: SPACE_SIZE,
                avoidance_radius: AVOIDANCE_RADIUS,
                avoidance_force: AVOIDANCE_FORCE,
                max_avoidance_force: MAX_AVOIDANCE_FORCE,
                wander_force: WANDER_FORCE,
                wander_angle_range: WANDER_ANGLE_RADIANS,
                wander_frequency: WANDER_FREQUENCY,

                flocking_radius: COHESION_RADIUS,
                cohesion_force: COHESION_FORCE,
                max_cohesion_force: MAX_COHESION_FORCE,
                alignment_force: 1.0,
                max_alignment_force: 2.0,

                max_angular_acceleration: 0.1,

                combined_drag_coefficient: COMBINED_DRAG_COEFFICIENT,
            })
            .add_systems(Startup, (add_boids, add_rendering))
            .add_systems(Update, (
                apply_flock_info,
                (
                    (apply_avoidance, apply_wander, apply_cohesion),
                    (apply_alignment, apply_drag),
                    add_velocity_to_position,
                    set_pos_vel_to_transform
                ).chain(),
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