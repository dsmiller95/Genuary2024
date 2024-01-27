use crate::seed_sim::consts::MAX_STEM_LENGTH;
use crate::seed_sim::prelude::{Organ, Stem};

impl Organ{
    pub fn get_generated_organs(&self) -> impl Iterator<Item=Organ>{
        let production = match self{
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
            Organ::Leaf => vec![Organ::Flower],
            Organ::Flower => vec![Organ::Fruit],
            Organ::Fruit => vec![Organ::Root],
            Organ::Root => vec![],
        };
        production.into_iter()
    }
}