use bevy::prelude::*;

pub const BOID_N: usize = 1000;
pub const SPACE_SIZE: usize = 500;
pub const SPAWN_VEL: f32 = 10.0;
pub const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const BOID_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
pub const BOID_SIZE: f32 = 10.0;

pub const AVOIDANCE_RADIUS: f32 = 50.0;
pub const AVOIDANCE_FORCE: f32 = 0.1;
pub const WANDER_FORCE: f32 = 0.01;
pub const WANDER_ANGLE_RADIANS: f32 = 0.01;
pub const WANDER_FREQUENCY: f32 = 1.0;
pub const COHESION_RADIUS: f32 = 100.0;
pub const COHESION_FORCE: f32 = 0.05;
pub const COMBINED_DRAG_COEFFICIENT: f32 = 0.01;