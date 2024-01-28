use crate::seed_sim::prelude::*;

#[derive(Debug, Clone)]
pub struct ParentRetargetFull {
    pub from: Entity,
    pub to: Entity,
}

#[derive(Resource)]
pub struct OrganParentRetargetingResources {
    pub parent_retargets: Option<Vec<ParentRetargetFull>>,
}

pub fn parent_retargeting(
    mut parent_retarget_res: ResMut<OrganParentRetargetingResources>,
    mut query: Query<(Entity, &SpawnStatus, &mut OrganRelations)>)
{
    let mut retarget_list = match parent_retarget_res.parent_retargets.take() {
        Some(retarget_list) => retarget_list,
        None => return,
    };

    retarget_list.sort_by_key(|a| a.from);

    for (_, spawn_status, mut organ_relations) in query.iter_mut() {
        match spawn_status.0 {
            SpawnedTime::ThisFrame => continue,
            SpawnedTime::OlderFrame => { },
        }
        let parent = match organ_relations.parent {
            Some(parent) => parent,
            None => continue,
        };
        let parent_retarget = retarget_list.binary_search_by_key(&parent, |a| a.from);
        let parent_retarget_index = match parent_retarget {
            Ok(parent_retarget) => parent_retarget,
            Err(_) => continue,
        };
        let replacement_parent = &retarget_list[parent_retarget_index].to;
        organ_relations.parent = Some(*replacement_parent);
    }
}

#[derive(Resource, Debug)]
pub struct PrintTimer(pub Timer);
pub fn print_parent_relationships(
    time: Res<Time>,
    mut timer: ResMut<PrintTimer>,
    query: Query<(Entity, &SpawnStatus, &OrganRelations, &EntityOrgan, &Transform)>)
{
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    for (self_entity, spawned, organ_relations, organ, transform) in query.iter() {
        println!("{:?} -> {:?}\t{:?}\t{:?}\t{:?}", self_entity, organ_relations.parent, spawned.0, organ.organ, transform);
    }
}

pub fn update_spawn_status_end_frame(mut query: Query<&mut SpawnStatus>)
{
    for mut spawn_status in query.iter_mut() {
        match spawn_status.0 {
            SpawnedTime::ThisFrame => {
                spawn_status.0 = SpawnedTime::OlderFrame;
            },
            SpawnedTime::OlderFrame => { },
        }
    }
}