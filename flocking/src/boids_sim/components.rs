use super::prelude::*;


#[derive(Resource)]
pub struct PrintTimer(pub Timer);

#[derive(Component)]
pub struct Boid;

#[derive(Component, Debug)]
pub struct Position {
    x: f32,
    y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn add_velocity(&mut self, velocity: &Velocity, time: &Time) {
        self.x += velocity.x * time.delta_seconds();
        self.y += velocity.y * time.delta_seconds();
    }
    pub fn set_transform(&self, transform: &mut Transform) {
        transform.translation.x = self.x;
        transform.translation.y = self.y;
    }
}


#[derive(Component, Debug)]
pub struct Velocity {
    x: f32,
    y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}