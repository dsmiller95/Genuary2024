use std::num::NonZeroUsize;
use super::prelude::*;

#[derive(Resource)]
pub struct PrintTimer(pub Timer);

#[derive(Resource)]
pub struct BoidBehavior {
    pub space_size: usize,
    pub avoidance_radius: f32,
    pub avoidance_force: f32,
    pub max_avoidance_force: f32,

    pub wander_force: f32,
    // units are radians
    pub wander_angle_range: f32,
    pub wander_frequency: f32,

    pub cohesion_radius: f32,
    pub cohesion_force: f32,
    pub max_cohesion_force: f32,


    // full drag formula is 1/2 * p * v^2 * C * A
    // where p is the density of the fluid, v is the velocity, C is the drag coefficient, and A is the cross-sectional area
    // combined_drag_coefficient = 1/2 * p * C * A
    pub combined_drag_coefficient: f32,
}

#[derive(Component)]
pub struct Boid;

#[derive(Component)] // seed is between 0 and 1
pub struct BoidSeed(pub f32);
#[derive(Component)]
pub struct BoidFlockInfo{
    center: Vec2,
    velocity: Vec2,
    count: usize,
}

impl BoidFlockInfo{
    pub fn new(position: &Position, velocity: &Velocity) -> Self{
        Self{
            center: position.vec,
            velocity: velocity.vec,
            count: 1,
        }
    }

    pub fn append_boid(&mut self, position: &Position, velocity: &Velocity){
        self.center += position.vec;
        self.velocity += velocity.vec;
        self.count += 1;
    }

    pub fn reset(&mut self, position: &Position, velocity: &Velocity){
        self.center = position.vec;
        self.velocity = velocity.vec;
        self.count = 1;
    }

    pub fn average_velocity(&self) -> Vec2{
        self.velocity / self.count as f32
    }

    pub fn average_position(&self) -> Vec2{
        self.center / self.count as f32
    }
}

#[derive(Component, Debug)]
pub struct Position {
    pub vec: Vec2,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { vec: Vec2::new(x, y) }
    }
    pub fn add_velocity(&mut self, velocity: &Velocity, time: &Time) {
        self.vec += velocity.vec * time.delta_seconds();
    }
    pub fn set_transform(&self, transform: &mut Transform) {
        transform.translation.x = self.vec.x;
        transform.translation.y = self.vec.y;
    }
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub vec: Vec2,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self {vec: Vec2::new(x, y) }
    }
}