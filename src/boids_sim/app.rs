use crate::boids_sim::components::*;
use crate::boids_sim::update_systems::*;
use super::prelude::*;

pub trait CanAddBoids {
    fn add_boids_app(&mut self) -> &mut App;
}
impl CanAddBoids for App {
    fn add_boids_app(&mut self) -> &mut Self {
        self
            .add_systems(Startup, add_boids)
            .add_systems(Update, hello_world)
            .add_systems(Update, add_velocity_to_position)
            .add_systems(Update, print_positions)
    }
}

fn hello_world(){
    println!("Hello World");
}

fn add_boids(mut commands: Commands) {
    let mut rng = SmallRng::from_entropy();
    for _ in 0..BOID_N {
        add_boid_to_world(&mut commands, &mut rng, 100.0, 10.0);
    }
    println!("Added {} boids", BOID_N);
}

fn add_boid_to_world(commands: &mut Commands, rng: &mut SmallRng, pos_max: f32, vel_max: f32) {
    let x = rng.gen_range(-pos_max..pos_max);
    let y = rng.gen_range(-pos_max..pos_max);
    let vx = rng.gen_range(-vel_max..vel_max);
    let vy = rng.gen_range(-vel_max..vel_max);

    commands.spawn((Boid, Position::new(x, y), Velocity::new(vx, vy)));
}