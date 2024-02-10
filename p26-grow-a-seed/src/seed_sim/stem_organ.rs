use std::f32::consts::PI;
use crate::seed_sim::consts::{EPSILON, OrganGenerationConsts};
use crate::seed_sim::organ_impls::{GeneratedEntityReference, GenerationResult, ParentRetarget, SpawnedOrgan};
use crate::seed_sim::prelude::*;

impl Stem {
    pub fn extend_up_to_max(&mut self, extra_len: f32, max_len: f32, segment_len: f32) -> GeneratedStem {
        let current_total_len = self.generated_len(segment_len);
        let new_total_len = (current_total_len + extra_len).min(max_len);
        let actual_increment = new_total_len - current_total_len;
        if actual_increment <= EPSILON {
            return GeneratedStem{ new_segments: 0, did_grow: false }
        }

        let new_partial_len = self.partial_length + actual_increment;
        let new_segments = (new_partial_len / segment_len).floor() as u8;
        self.partial_length = new_partial_len - (new_segments as f32 * segment_len);
        self.generated_segments += new_segments;
        GeneratedStem{ new_segments, did_grow: true }
    }

    pub fn generated_len(&self, segment_len: f32) -> f32 {
        self.partial_length + (self.generated_segments as f32 * segment_len)
    }

    pub fn get_production_result(&mut self, self_entity: Entity, self_parent: Option<Entity>, consts: &OrganGenerationConsts) -> (Option<Organ>, GenerationResult){
        enum GrowthState{
            FullyGrown, Growing
        }

        let growth_increment = consts.stem_growth_per_step;
        let growth_result = self.extend_up_to_max(growth_increment, consts.max_stem_length, consts.segment_len);

        // if we didn't grow, then we have completed our lifecycle, and replace ourselves.
        if !growth_result.did_grow {
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
                    organ: Organ::Crook{ angle: -PI / 2.0 },
                    parent: Some(GeneratedEntityReference::External(self_entity)),
                },
                SpawnedOrgan{
                    organ: Organ::Leaf,
                    parent: Some(GeneratedEntityReference::Internal(4)),
                },
            ];
            return (
                Some(Organ::StemSeg),
                GenerationResult {
                    spawned,
                    children_point_to: ParentRetarget::Changed(GeneratedEntityReference::Internal(1)),
                    parent_point_to: ParentRetarget::Unchanged,
                }
            );
        }

        if growth_result.new_segments <= 0 {
            return Default::default();
        }

        let mut spawned = Vec::with_capacity(growth_result.new_segments as usize);

        for i in 0..growth_result.new_segments {
            let parent = match i {
                0 => match self_parent {
                    Some(parent) => Some(GeneratedEntityReference::External(parent)),
                    None => None,
                },
                _ => Some(GeneratedEntityReference::Internal(i as usize - 1)),
            };
            spawned.push(SpawnedOrgan{
                organ: Organ::StemSeg,
                parent: parent,
            });
        }

        let self_parent_points_to = ParentRetarget::Changed(GeneratedEntityReference::Internal(growth_result.new_segments as usize - 1));

        return (
            None,
            GenerationResult {
                spawned,
                children_point_to: ParentRetarget::Unchanged,
                parent_point_to: self_parent_points_to,
            }
        );
    }
}

struct GeneratedStem{
    new_segments: u8,
    did_grow: bool,
}

#[cfg(test)]
mod test {
    use float_cmp::{approx_eq, assert_approx_eq};
    use super::*;

    #[test]
    fn when_growing_directly__past_organ_segment__increments_organ_segment(){
        let mut stem = Stem::default();
        let result = stem.extend_up_to_max(1.2, 10.0, 1.0);
        assert_eq!(result.new_segments, 1);
        assert_eq!(result.did_grow, true);
        assert!(approx_eq!(f32, stem.partial_length, 0.2));
        assert_eq!(stem.generated_segments, 1);
    }

    #[test]
    fn when_growing_up_to_max__caps_at_max(){
        let mut stem = Stem::default();
        stem.partial_length = 0.4;
        stem.generated_segments = 1;
        let result = stem.extend_up_to_max(4.0, 5.0, 1.0);
        assert_eq!(result.new_segments, 4);
        assert_eq!(result.did_grow, true);
        assert_approx_eq!(f32, stem.partial_length, 0.0);
        assert_eq!(stem.generated_segments, 5);
    }

    fn assert_organ_eq(organ: Organ, expected: Organ){
        assert_approx_eq!(&Organ, &organ, &expected);
    }
    #[test]
    fn when_growth_less_than_segment_len__grows_without_spawning(){
        let mut consts: OrganGenerationConsts = Default::default();
        consts.stem_growth_per_step = 0.2;

        let mut organ = Organ::Stem(Stem::default());
        let parent = Entity::from_raw(0);
        let self_entity = Entity::from_raw(1);
        let result = organ.get_generated_organ_commands(self_entity, Some(parent), &consts);

        assert_eq!(result.spawned.len(), 0);
        assert_eq!(result.children_point_to, ParentRetarget::Unchanged);
        assert_eq!(result.parent_point_to, ParentRetarget::Unchanged);
        assert_organ_eq(organ.clone(), Organ::Stem(Stem{ partial_length: 0.2, generated_segments: 0 }));
    }

    #[test]
    fn when_growth_more_than_segment_len__spawns_segment(){
        let mut consts: OrganGenerationConsts = Default::default();
        consts.stem_growth_per_step = 1.2;
        consts.segment_len = 1.0;

        let mut organ = Organ::Stem(Stem::default());
        let parent = Entity::from_raw(0);
        let self_entity = Entity::from_raw(1);
        let result = organ.get_generated_organ_commands(self_entity, Some(parent), &consts);

        assert_eq!(result.spawned.len(), 1);
        assert_eq!(result.children_point_to, ParentRetarget::Unchanged);
        assert_eq!(result.parent_point_to, ParentRetarget::Changed(GeneratedEntityReference::Internal(0)));
        assert_organ_eq(organ, Organ::Stem(Stem{ partial_length: 0.2, generated_segments: 1 }));

        let spawned_seg = result.spawned.first().unwrap();
        assert_eq!(spawned_seg.organ, Organ::StemSeg);
        assert_eq!(spawned_seg.parent, Some(GeneratedEntityReference::External(parent)));
    }

    #[test]
    fn when_has_no_parent__and_spawns_segment__segment_parent_is_none(){
        let mut consts: OrganGenerationConsts = Default::default();
        consts.stem_growth_per_step = 1.2;
        consts.segment_len = 1.0;

        let mut organ = Organ::Stem(Stem::default());
        let self_entity = Entity::from_raw(1);
        let result = organ.get_generated_organ_commands(self_entity, None, &consts);

        assert_eq!(result.spawned.len(), 1);
        assert_eq!(result.children_point_to, ParentRetarget::Unchanged);
        assert_eq!(result.parent_point_to, ParentRetarget::Changed(GeneratedEntityReference::Internal(0)));
        assert_organ_eq(organ, Organ::Stem(Stem{ partial_length: 0.2, generated_segments: 1 }));

        let spawned_seg = result.spawned.first().unwrap();
        assert_eq!(spawned_seg.organ, Organ::StemSeg);
        assert_eq!(spawned_seg.parent, None);
    }

    #[test]
    fn when_growth_more_than_segment_len__spawns_multiple_segments(){
        let mut consts: OrganGenerationConsts = Default::default();
        consts.stem_growth_per_step = 2.3;
        consts.segment_len = 1.0;

        let mut organ = Organ::Stem(Stem::default());
        let parent = Entity::from_raw(0);
        let self_entity = Entity::from_raw(1);
        let result = organ.get_generated_organ_commands(self_entity, Some(parent), &consts);

        assert_eq!(result.spawned.len(), 2);
        assert_eq!(result.children_point_to, ParentRetarget::Unchanged);
        assert_eq!(result.parent_point_to, ParentRetarget::Changed(GeneratedEntityReference::Internal(1)));
        assert_organ_eq(organ, Organ::Stem(Stem{ partial_length: 0.3, generated_segments: 2 }));
    }

    #[test]
    fn when_growth_more_than_segment_len__spawns_multiple_segments__and_grows_partial_length(){
        let mut consts: OrganGenerationConsts = Default::default();
        consts.stem_growth_per_step = 0.8;
        consts.segment_len = 0.5;

        let mut organ = Organ::Stem(Stem{ partial_length: 0.4, generated_segments: 2 });
        let parent = Entity::from_raw(0);
        let self_entity = Entity::from_raw(1);
        let result = organ.get_generated_organ_commands(self_entity, Some(parent), &consts);

        assert_eq!(result.spawned.len(), 2);
        assert_eq!(result.children_point_to, ParentRetarget::Unchanged);
        assert_eq!(result.parent_point_to, ParentRetarget::Changed(GeneratedEntityReference::Internal(1)));
        assert_organ_eq(organ, Organ::Stem(Stem{ partial_length: 0.2, generated_segments: 4 }));
    }

    #[test]
    fn when_growth_hits_max_length__grows_to_max(){
        let mut consts: OrganGenerationConsts = Default::default();
        consts.stem_growth_per_step = 1.7;
        consts.segment_len = 1.0;
        consts.max_stem_length = 5.0;

        let mut organ = Organ::Stem(Stem{ partial_length: 0.6, generated_segments: 4 });
        let parent = Entity::from_raw(0);
        let self_entity = Entity::from_raw(1);
        let result = organ.get_generated_organ_commands(self_entity, Some(parent), &consts);

        assert_eq!(result.spawned.len(), 1);
        assert_eq!(result.children_point_to, ParentRetarget::Unchanged);
        assert_eq!(result.parent_point_to, ParentRetarget::Changed(GeneratedEntityReference::Internal(0)));
        assert_organ_eq(organ, Organ::Stem(Stem{ partial_length: 0.0, generated_segments: 5 }));
    }
}