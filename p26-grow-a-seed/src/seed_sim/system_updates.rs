use bevy::prelude::{Query, Res, Time};
use crate::seed_sim::organ_impls::{GeneratedEntityReference, ParentRetarget, SpawnedOrgan};
use crate::seed_sim::parent_retargeting::*;
use crate::seed_sim::plant_bundle::OrganBundle;
use super::prelude::*;

pub fn organ_production(
    time: Res<Time>,
    mut parent_retarget_res: ResMut<OrganParentRetargetingResources>,
    mut query: Query<(Entity, &mut EntityOrgan, &mut SeedTimer, &Transform)>,
    mut commands: Commands)
{
    let mut parent_retargets = Vec::new();
    for (self_entity, mut organ, mut timer, transform) in query.iter_mut() {
        if !timer.0.tick(time.delta()).just_finished() {
            continue;
        }

        let (generations, parent_retarget) = organ.organ.get_generated_organ_commands(self_entity);

        let spawned_entities = spawn_organs(generations, transform.clone(), &mut commands);
        match parent_retarget {
            ParentRetarget::Changed(parent) => {
                parent_retargets.push(ParentRetargetFull{
                    from: self_entity,
                    to: match parent {
                        GeneratedEntityReference::Internal(index) => spawned_entities[index],
                        GeneratedEntityReference::External(entity) => entity,
                    }
                })
            },
            ParentRetarget::Unchanged => {}
        }
    }

    match parent_retarget_res.parent_retargets {
        Some(_) => panic!("Parent retargets already set this frame"),
        None => {}
    };
    parent_retarget_res.parent_retargets = match parent_retargets.len() {
        0 => None,
        _ => Some(parent_retargets),
    };
}

fn spawn_organs(
    spawned_organs: Vec<SpawnedOrgan>,
    origin: Transform,
    commands: &mut Commands) -> Vec<Entity> {
    let mut spawned_organ_entities = Vec::with_capacity(spawned_organs.len());
    for spawned_organ in spawned_organs {
        let parent_entity = match spawned_organ.parent {
            Some(GeneratedEntityReference::Internal(parent_index)) => {
                if spawned_organ_entities.len() <= parent_index {
                    panic!("Spawned organ parent index out of bounds. Spawned organs must form a root-first Directed Acyclic Graph.")
                }
                Some(spawned_organ_entities[parent_index])
            },
            Some(GeneratedEntityReference::External(parent_entity)) => {
                Some(parent_entity)
            },
            None => None,
        };
        let bundle = OrganBundle::new_from_organ(spawned_organ.organ, parent_entity, origin);
        let entity = commands.spawn(bundle).id();
        spawned_organ_entities.push(entity);
    }

    spawned_organ_entities
}
