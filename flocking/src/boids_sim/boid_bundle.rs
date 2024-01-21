use std::f32::consts::PI;
use super::prelude::*;
use crate::boids_sim::components::*;

#[derive(Bundle)]
pub struct BoidBundle{
    boid: Boid,
    boid_seed: BoidSeed,
    flock: BoidFlockInfo,
    position: Position,
    rotation: Rotation,
    speed: Speed,
    rotational_velocity: RotationalVelocity,
    //steering_force: SteeringForce,
    sprite_bundle: SpriteBundle,
}

impl BoidBundle{
    pub fn new(rng: &mut SmallRng, pos_max: f32, vel_max: f32, vel_variance: f32) -> Self{
        let x = rng.gen_range(-pos_max..pos_max);
        let y = rng.gen_range(-pos_max..pos_max);
        let s = rng.gen_range(vel_max-vel_variance..vel_max+vel_variance);

        let position = Position::new(x, y);
        let rotation = Rotation{radians: rng.gen_range(0.0..2.0*PI)};
        let speed = Speed{pixels_per_second: s };

        Self{
            boid: Boid,
            boid_seed: BoidSeed(SmallRng::from_rng(rng).unwrap()),
            flock: BoidFlockInfo::new(&position, &rotation),
            position,
            rotation,
            speed,
            rotational_velocity: RotationalVelocity{radians_per_second: 0.0},
            sprite_bundle: SpriteBundle{
                transform: Transform::from_xyz(x, y, 0.0),
                sprite: Sprite{
                    color: BOID_COLOR,
                    custom_size: Some(Vec2::new(BOID_SIZE, BOID_SIZE/4.0)),
                    ..Default::default()
                },
                ..Default::default()
            }
        }
    }
}