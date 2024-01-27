use bevy::prelude::{Query, Res, Time};
use super::prelude::*;

pub fn grow_seed(time: Res<Time>, mut query: Query<(&mut Seed, &mut SeedTimer)>) {
    for (mut seed, mut timer) in query.iter_mut() {
        if !timer.0.tick(time.delta()).just_finished() {
            continue;
        }

        seed.steps += 1;
        seed.organs = seed.organs.iter()
            .map(|organ| { organ.get_generated_organs() })
            .flatten().collect();

        println!("Seed: {:?}", seed)
    }
}