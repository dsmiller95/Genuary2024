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

    // units are radians
    pub wander_angle_range: f32,

    ////// Flocking //////
    pub flocking_radius: f32,

    pub cohesion_force: f32,
    pub max_cohesion_force: f32,

    /// units: rad/(s*rad)
    pub alignment_force: f32,
    /// units: rad
    pub max_alignment_force: f32,
    ////// Flocking //////

    // units are rad/s^2
    pub max_angular_acceleration: f32,

    // full drag formula is 1/2 * p * v^2 * C * A
    // where p is the density of the fluid, v is the velocity, C is the drag coefficient, and A is the cross-sectional area
    // combined_drag_coefficient = 1/2 * p * C * A
    pub combined_drag_coefficient: f32,
}

#[derive(Component)]
pub struct Boid;

#[derive(Component)] // seed is between 0 and 1
pub struct BoidSeed(pub SmallRng);
#[derive(Component)]
pub struct BoidFlockInfo{
    center: Vec2,
    direction: Vec2,
    count: usize,
}

impl BoidFlockInfo{
    pub fn new(position: &Position, rotation: &Rotation) -> Self{
        Self{
            center: position.vec,
            direction: rotation.vec(),
            count: 1,
        }
    }

    pub fn append_boid(&mut self, position: &Position, rotation: &Rotation){
        self.center += position.vec;
        self.direction += rotation.vec();
        self.count += 1;
    }

    pub fn reset(&mut self, position: &Position, rotation: &Rotation){
        self.center = position.vec;
        self.direction = rotation.vec();
        self.count = 1;
    }

    pub fn average_direction(&self) -> Vec2{
        self.direction / self.count as f32
    }

    pub fn average_position(&self) -> Vec2{
        self.center / self.count as f32
    }
}

#[derive(Component, Debug)]
pub struct Position {
    pub vec: Vec2,
}
#[derive(Component, Debug)]
pub struct Rotation {
    pub radians: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { vec: Vec2::new(x, y) }
    }
}

impl Rotation {
    pub fn vec(&self) -> Vec2 {
        Vec2::from_angle(self.radians)
    }
}

#[derive(Component, Debug)]
pub struct Speed {
    pub pixels_per_second: f32,
}
#[derive(Component, Debug)]
pub struct RotationalVelocity {
    pub radians_per_second: f32,
}

#[derive(Component, Debug)]
pub struct SteeringForce {
    // units are rad/s^2
    pub angular_acceleration: f32,
}
