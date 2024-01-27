use std::fmt::Debug;
use bevy::ecs::query::WorldQuery;
use bevy::prelude::Transform;
use super::prelude::*;

pub fn propagate_custom_transforms_inplace(
    mut query: Query<(Entity, &mut Transform, &OrganRelations, &EntityOrgan)>
){
    let count = query.iter().count();
    let _span = info_span!("propagate_transforms", name="propagate_transforms", num=count).entered();
    fold_up_unordered(
        query,
        |organ_relations| organ_relations.parent,
        |parent_transform, entity_organ| {
        let organ_transform = entity_organ.organ.get_transformation();
        Some(parent_transform.mul_transform(organ_transform))
    });
}

fn fold_up_unordered<T1: Component + Debug, T2: WorldQuery, T3: WorldQuery, F1, F2>(
    mut query: Query<(Entity, &mut T1, T2, T3)>,
    select_dependency: F1,
    produce_new_value: F2,
)
    where F1: Fn(T2::Item<'_>) -> Option<Entity>,
          F2: Fn(&T1, T3::Item<'_>) -> Option<T1>,
{
    // Safety: Will panic if an entity selects itself as its own parent
    let unsafe_query = unsafe {
        query.iter_unsafe()
    };
    for (_e, mut _transform, organ_relations, entity_organ) in  unsafe_query {
        let dependency = match select_dependency(organ_relations) {
            Some(parent) => parent,
            None => continue,
        };
        if dependency == _e {
            panic!("Entity {:?} is its own parent", _e);
        }

        let dependency_component = match query.get(dependency) {
            Ok((_, trans, _, _)) => trans,
            Err(_) => continue,
        };
        let new_value = produce_new_value(dependency_component, entity_organ);
        match new_value {
            Some(new_value) => *_transform = new_value,
            None => continue,
        }
    }

}