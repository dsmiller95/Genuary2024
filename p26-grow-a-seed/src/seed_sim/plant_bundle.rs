use std::time::Duration;
use super::prelude::*;

#[derive(Bundle)]
pub struct OrganBundle{
    sprite_bundle: SpriteBundle,
    organ: EntityOrgan,
    organ_relations: OrganRelations,
    spawn_status: SpawnStatus,
    seed_timer: SeedTimer,
}
impl OrganBundle {
    pub fn new(rng: &mut SmallRng, pos_max: f32) -> Self {
        let x = rng.gen_range(-pos_max..pos_max);
        let y = rng.gen_range(-pos_max..pos_max);
        let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);
        timer.tick(Duration::from_secs_f32(rng.gen_range(0.0..1.0)));
        Self {
            organ: EntityOrgan{
                organ: Organ::Stem(Stem { length: 0.0 }),
            },
            organ_relations: OrganRelations{ parent: None },
            spawn_status: SpawnStatus(SpawnedTime::ThisFrame),
            seed_timer: SeedTimer(timer),
            sprite_bundle: SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0).with_scale(Vec3::new(5.0, 10.0, 5.0)),
                sprite: Sprite {
                    color: Color::rgb(0.0, 1.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }

    pub fn new_from_organ(organ: Organ, parent: Option<Entity>, transform: Transform) -> Self {
        Self {
            organ: EntityOrgan{ organ, },
            organ_relations: OrganRelations{ parent },
            spawn_status: SpawnStatus(SpawnedTime::ThisFrame),
            seed_timer: SeedTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
            sprite_bundle: SpriteBundle {
                transform,
                sprite: Sprite {
                    color: Color::rgb(0.0, 1.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}
