use super::prelude::*;

#[derive(Resource)]
pub struct PrintTimer(pub Timer);

#[derive(Resource)]
pub struct BoidBehavior {
    pub space_size: usize,
    pub avoidance_radius: f32,
    pub avoidance_force: f32,
    pub wander_force: f32,
    pub wander_frequency: f32,
}

#[derive(Component)]
pub struct Boid;

#[derive(Component)] // seed is between 0 and 1
pub struct BoidSeed(pub f32);

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