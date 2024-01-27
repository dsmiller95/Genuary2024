use std::collections::HashMap;
use bevy::prelude::Transform;
use super::prelude::*;

pub fn propagate_custom_transforms(
    mut query: Query<(Entity, &mut Transform, &OrganRelations, &EntityOrgan)>
)
{
    let mut changed_transforms : HashMap<Entity, Transform> = HashMap::new();

    for (_e, _transform, organ_relations, entity_organ) in query.iter() {
        let parent = match organ_relations.parent {
            Some(parent) => parent,
            None => continue,
        };

        let parent_transform = match query.get(parent) {
            Ok((_, trans, _, _)) => trans,
            Err(_) => continue,
        };

        let organ_transform = entity_organ.organ.get_transformation();
        changed_transforms.insert(_e, parent_transform.mul_transform(organ_transform));
    }

    for (entity, mut transform, _, _) in query.iter_mut() {
        let new_transform = match changed_transforms.get(&entity) {
            Some(new_transform) => new_transform,
            None => continue,
        };
        *transform = new_transform.clone();
    }
}
