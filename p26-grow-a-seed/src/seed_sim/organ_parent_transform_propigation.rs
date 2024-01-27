use std::collections::HashMap;
use bevy::prelude::Transform;
use super::prelude::*;

pub fn propagate_custom_transforms(
    mut query: Query<(Entity, &mut Transform, &OrganRelations, &EntityOrgan)>,
)
{
    let parent_transforms: HashMap<Entity, Transform> = query.iter()
        .map(|(entity, transform, _, _)| {
            (entity, transform.clone())
        }).collect();

    for (_e, mut transform, organ_relations, entity_organ) in query.iter_mut() {
        let parent = match organ_relations.parent {
            Some(parent) => parent,
            None => continue,
        };
        let parent_transform = match parent_transforms.get(&parent) {
            Some(transform) => transform,
            None => continue,
        };

        let organ_transform = entity_organ.organ.get_transformation();
        *transform = parent_transform.mul_transform(organ_transform);
    }
}
