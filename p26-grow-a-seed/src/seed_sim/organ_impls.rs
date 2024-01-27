use crate::seed_sim::plant_organs_resources::OrganResources;
use crate::seed_sim::prelude::*;

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

    pub fn get_transformation(&self) -> Transform{
        match self{
            Organ::Stem(stem) => {
                Transform::from_xyz(0.0, stem.length/10.0, 0.0)
            },
            Organ::Leaf => Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)),
            Organ::Flower => Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)),
            Organ::Fruit => Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)),
            Organ::Root => Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)),
        }
    }

    pub fn render_single(&self, origin: Transform, organ_resources: &OrganResources, commands: &mut Commands) {
        let organ_transform = self.get_transformation();
        let transform = origin.mul_transform(organ_transform);
        let color = match self{
            Organ::Stem(stem) => {
                let green = stem.length / MAX_STEM_LENGTH;
                Color::rgb(0.0, green, 0.0)
            }
            Organ::Leaf => Color::rgb(0.0, 1.0, 0.0),
            Organ::Flower => Color::rgb(1.0, 0.0, 0.0),
            Organ::Fruit => Color::rgb(1.0, 0.0, 1.0),
            Organ::Root => Color::rgb(0.0, 0.0, 1.0),
        };

        let mut bundle = organ_resources.stem_bundle.clone();
        bundle.sprite_bundle.transform = transform;
        bundle.sprite_bundle.sprite.color = color;
        commands.spawn(bundle);
    }

}

pub struct SpawnedOrgan{
    pub organ: Organ,
    pub parent: Option<GeneratedEntityReference>,
}

pub enum GeneratedEntityReference {
    Internal(usize),
    External(Entity)
}

/// Which entity to point all children to, if changed
pub enum ParentRetarget {
    Changed(GeneratedEntityReference),
    Unchanged,
}

impl Organ {
    pub fn get_generated_organ_commands(&mut self, self_entity: Entity) -> (Vec<SpawnedOrgan>, ParentRetarget) {
        return match self {
            Organ::Stem(ref mut stem) => {
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
                    (GrowthState::Growing, GrowthState::FullyGrown) => {
                        stem.length = MAX_STEM_LENGTH;
                        let spawned = vec![
                            SpawnedOrgan{
                                organ: Organ::Stem(Stem { length: 0.0 }),
                                parent: Some(GeneratedEntityReference::External(self_entity)),
                            },
                            SpawnedOrgan{
                                organ: Organ::Leaf,
                                parent: Some(GeneratedEntityReference::Internal(0)),
                            },
                        ];
                        (spawned, ParentRetarget::Changed(GeneratedEntityReference::Internal(1)))
                    },
                    (GrowthState::Growing, GrowthState::Growing) => {
                        stem.length = new_length;
                        (vec![], ParentRetarget::Unchanged)
                    },
                    (GrowthState::FullyGrown, GrowthState::FullyGrown) => {
                        stem.length = MAX_STEM_LENGTH;
                        (vec![], ParentRetarget::Unchanged)
                    },
                    (GrowthState::FullyGrown, GrowthState::Growing) =>
                        panic!("Cannot transition from fully grown to growing")
                }
            },
            Organ::Leaf => (vec![], ParentRetarget::Unchanged),
            Organ::Flower => (vec![], ParentRetarget::Unchanged),
            Organ::Fruit => (vec![], ParentRetarget::Unchanged),
            Organ::Root => (vec![], ParentRetarget::Unchanged),
        }
    }
}