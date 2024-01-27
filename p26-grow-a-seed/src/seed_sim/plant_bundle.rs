use std::time::Duration;
use super::prelude::*;

#[derive(Bundle)]
pub struct PlantBundle {
    seed: Seed,
    seed_timer: SeedTimer,
    sprite_bundle: SpriteBundle,
}

impl PlantBundle {
    pub fn new(rng: &mut SmallRng, pos_max: f32) -> Self {
        let x = rng.gen_range(-pos_max..pos_max);
        let y = rng.gen_range(-pos_max..pos_max);
        let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);
        timer.tick(Duration::from_secs_f32(rng.gen_range(0.0..1.0)));
        Self {
            seed: Seed {
                organs: vec![Organ::Stem(Stem { length: 0.0 })],
                steps: 0,
            },
            seed_timer: SeedTimer(timer),
            sprite_bundle: SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                sprite: Sprite {
                    color: Color::rgb(0.0, 1.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}