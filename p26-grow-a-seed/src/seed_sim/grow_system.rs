use bevy::prelude::{Query, Res, Time};
use super::prelude::*;

pub fn grow_seed(time: Res<Time>, mut query: Query<(&mut Seed, &mut SeedTimer)>) {
    for (mut seed, mut timer) in query.iter_mut() {
        if !timer.0.tick(time.delta()).just_finished() {
            continue;
        }

        seed.steps += 1;
        seed.organs = grow_organs(&seed.organs);

        println!("Seed: {:?}", seed)
    }
}

pub fn grow_organs(organs: &[Organ]) -> Vec<Organ> {

    organs.iter().map(|organ| {
        let result_seq = match organ {
            Organ::Stem(stem) => {
                enum GrowthState{
                    FullyGrown, Growing
                }
                let last_grow_state = match stem.length {
                    MAX_STEM_LENGTH.. => GrowthState::FullyGrown,
                    _ => GrowthState::Growing,
                };
                let new_length = stem.length + 1.0;
                let next_grow_state = match new_length {
                    MAX_STEM_LENGTH.. => GrowthState::FullyGrown,
                    _ => GrowthState::Growing,
                };
                match (last_grow_state, next_grow_state) {
                    (GrowthState::Growing, GrowthState::FullyGrown) =>
                        vec![
                            Organ::Stem(Stem {
                                length: MAX_STEM_LENGTH
                            }),
                            Organ::Stem(Stem {
                                length: 0.0,
                            }),
                            Organ::Leaf,
                        ],
                    (GrowthState::Growing, GrowthState::Growing) =>
                        vec![
                            Organ::Stem(Stem {
                                length: new_length
                            })
                        ],
                    (GrowthState::FullyGrown, GrowthState::FullyGrown) =>
                        vec![
                            Organ::Stem(Stem {
                                length: MAX_STEM_LENGTH
                            }),
                        ],
                    (GrowthState::FullyGrown, GrowthState::Growing) =>
                        panic!("Cannot transition from fully grown to growing")
                }
            },
            Organ::Leaf => {
                vec![
                    Organ::Leaf,
                ]
            },
            _ => todo!("implement grow_organs")
        };

        result_seq
    }).flatten().collect()
}