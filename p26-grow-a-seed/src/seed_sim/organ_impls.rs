use std::f32::consts::PI;
use crate::seed_sim::prelude::*;

impl Organ{
    pub fn get_transformation(&self) -> Transform{
        match self{
            Organ::Stem(stem) => {
                Transform::from_xyz(0.0, stem.length/5.0, 0.0)
            },
            Organ::Crook { angle } => {
                Transform::from_rotation(Quat::from_rotation_z(*angle))
            },
            Organ::Leaf => Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)),
            Organ::Flower => Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)),
            Organ::Fruit => Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)),
            Organ::Root => Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)),
        }
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
                                organ: Organ::Crook{ angle: 0.5 },
                                parent: Some(GeneratedEntityReference::External(self_entity)),
                            },
                            SpawnedOrgan{
                                organ: Organ::Crook{ angle: -0.5 },
                                parent: Some(GeneratedEntityReference::External(self_entity)),
                            },
                            SpawnedOrgan{
                                organ: Organ::Stem(Stem { length: 0.0 }),
                                parent: Some(GeneratedEntityReference::Internal(0)),
                            },
                            SpawnedOrgan{
                                organ: Organ::Stem(Stem { length: 0.0 }),
                                parent: Some(GeneratedEntityReference::Internal(1)),
                            },
                            SpawnedOrgan{
                                organ: Organ::Crook{ angle: -PI / 2.0 },
                                parent: Some(GeneratedEntityReference::External(self_entity)),
                            },
                            SpawnedOrgan{
                                organ: Organ::Leaf,
                                parent: Some(GeneratedEntityReference::Internal(4)),
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
            _ => (vec![], ParentRetarget::Unchanged),
        }
    }
}