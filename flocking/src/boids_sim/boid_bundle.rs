use super::prelude::*;
use crate::boids_sim::components::*;

#[derive(Bundle)]
pub struct BoidBundle{
    boid: Boid,
    position: Position,
    velocity: Velocity,
    sprite_bundle: SpriteBundle,
}

impl BoidBundle{
    pub fn new(rng: &mut SmallRng, pos_max: f32, vel_max: f32) -> Self{
        let x = rng.gen_range(-pos_max..pos_max);
        let y = rng.gen_range(-pos_max..pos_max);
        let vx = rng.gen_range(-vel_max..vel_max);
        let vy = rng.gen_range(-vel_max..vel_max);

        Self{
            boid: Boid,
            position: Position::new(x, y),
            velocity: Velocity::new(vx, vy),
            sprite_bundle: SpriteBundle{
                transform: Transform::from_xyz(x, y, 0.0),
                sprite: Sprite{
                    color: BOID_COLOR,
                    custom_size: Some(Vec2::new(BOID_SIZE, BOID_SIZE)),
                    ..Default::default()
                },
                ..Default::default()
            }
        }
    }
}