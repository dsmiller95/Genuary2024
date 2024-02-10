use crate::seed_sim::prelude::*;

impl Organ{
    pub fn get_transformation(&self) -> Transform{
        match self{
            Organ::Seed => Transform::IDENTITY,
            Organ::Stem(stem) => {
                Transform::from_xyz(0.0, stem.partial_length, 0.0)
            },
            Organ::Crook { angle } => {
                Transform::from_rotation(Quat::from_rotation_z(*angle))
            },
            Organ::Leaf => Transform::IDENTITY,
            Organ::Flower => Transform::IDENTITY,
            Organ::Fruit => Transform::IDENTITY,
            Organ::Root { rotation } => {
                Transform::from_rotation(Quat::from_rotation_z(*rotation))
            },
            Organ::Origin => Transform::IDENTITY,
            Organ::StemSeg => Transform::from_xyz(0.0, SEGMENT_LEN, 0.0),
            Organ::EventualBranch{..} => Transform::IDENTITY,
        }
    }

    pub fn get_local_transformation(&self) -> Transform{
        match self{
            Organ::Seed => Transform::IDENTITY,
            Organ::Stem(stem) => {
                Transform::from_scale(Vec3::new(1.0, stem.partial_length, 1.0))
            },
            Organ::Crook{..} => Transform::IDENTITY,
            Organ::Leaf => Transform::IDENTITY,
            Organ::Flower => Transform::IDENTITY,
            Organ::Fruit => Transform::IDENTITY,
            Organ::Root{..} => Transform::IDENTITY,
            Organ::Origin => Transform::IDENTITY,
            Organ::StemSeg => Transform::IDENTITY,
            Organ::EventualBranch{..} => Transform::IDENTITY,
        }
    }
}

pub struct SpawnedOrgan{
    pub organ: Organ,
    pub parent: Option<GeneratedEntityReference>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum GeneratedEntityReference {
    /// Index into the list of spawned entities
    Internal(usize),
    /// Reference to an entity that was not spawned this frame
    External(Entity)
}

/// Which entity to point all children to, if changed
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParentRetarget {
    // point all parents of the current entity to this entity
    Changed(GeneratedEntityReference),
    Unchanged,
}

pub struct GenerationResult{
    pub spawned: Vec<SpawnedOrgan>,
    /// a retarget operation to apply to all children of the current entity
    /// When `Changed`, will move all children of the current node to the new parent
    pub children_point_to: ParentRetarget,
    /// a retarget operation to apply to -this- entity.
    /// When `Changed`, will move the current entity to the new parent
    pub parent_point_to: ParentRetarget,
}

impl Default for GenerationResult {
    fn default() -> Self {
        GenerationResult {
            spawned: vec![],
            children_point_to: ParentRetarget::Unchanged,
            parent_point_to: ParentRetarget::Unchanged,
        }
    }
}

impl Organ {
    pub fn get_generated_organ_commands(&mut self, self_entity: Entity, self_parent: Option<Entity>, consts: &OrganGenerationConsts) -> GenerationResult {
        return match self {
            Organ::Seed => {
                *self = Organ::Origin;
                let spawned = vec![
                    SpawnedOrgan{
                        organ: Organ::Root{rotation: 0.0 },
                        parent: Some(GeneratedEntityReference::External(self_entity)),
                    },
                    SpawnedOrgan{
                        organ: Organ::EventualBranch{ steps_till_branch: 0 },
                        parent: Some(GeneratedEntityReference::Internal(0)),
                    },
                ];
                GenerationResult {
                    spawned,
                    children_point_to: ParentRetarget::Changed(GeneratedEntityReference::Internal(1)),
                    parent_point_to: ParentRetarget::Unchanged,
                }
            }
            Organ::Stem(ref mut stem) => {
                let (replacement_organ, gen_result) = stem.get_production_result(self_entity, self_parent, consts);
                if let Some(replacement_organ) = replacement_organ {
                    *self = replacement_organ;
                }
                gen_result
            },
            Organ::Root{ref mut rotation} => {
                *rotation += 0.01;
                Default::default()
            },
            Organ::EventualBranch { steps_till_branch: 0 } => {
                *self = Organ::Flower;
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
                        organ: Organ::Stem(Default::default()),
                        parent: Some(GeneratedEntityReference::Internal(0)),
                    },
                    SpawnedOrgan{
                        organ: Organ::Stem(Default::default()),
                        parent: Some(GeneratedEntityReference::Internal(1)),
                    },
                    SpawnedOrgan{
                        organ: Organ::EventualBranch{ steps_till_branch: consts.steps_till_eventual_branch },
                        parent: Some(GeneratedEntityReference::Internal(2)),
                    },
                    SpawnedOrgan{
                        organ: Organ::EventualBranch{ steps_till_branch: consts.steps_till_eventual_branch },
                        parent: Some(GeneratedEntityReference::Internal(3)),
                    },
                ];
                GenerationResult {
                    spawned,
                    children_point_to: ParentRetarget::Changed(GeneratedEntityReference::Internal(4)),
                    parent_point_to: ParentRetarget::Unchanged,
                }
            },
            Organ::EventualBranch { steps_till_branch } => {
                *steps_till_branch -= 1;
                Default::default()
            },
            _ => default(),
        }
    }
}
