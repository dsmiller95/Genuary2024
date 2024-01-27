
use bevy::prelude::{Query, Res, Time};
use crate::seed_sim::plant_organs_resources::{OrganResources};
use super::prelude::*;

pub fn grow_seed(time: Res<Time>, mut query: Query<(&mut Seed, &mut SeedTimer)>) {
    for (mut seed, mut timer) in query.iter_mut() {
        if !timer.0.tick(time.delta()).just_finished() {
            continue;
        }

        seed.steps += 1;
        seed.organs = seed.organs.iter()
            .map(|organ| { organ.get_generated_organs() })
            .flatten().collect();

        println!("Seed: {:?}", seed)
    }
}

pub fn render_seed(organ_resources: Res<OrganResources>, mut query: Query<(&Seed, &Transform)>, mut commands: Commands) {
    let res_as_ref = organ_resources.as_ref();
    for (seed, transform) in query.iter_mut() {
        let origin = transform;
        render_branch(origin.clone(), &seed.organs, res_as_ref, &mut commands);
    }
}

fn render_branch(origin: Transform, organs: &[Organ], organ_resources: &OrganResources, commands: &mut Commands){
    let mut turtle_transform = origin;
    for organ in organs {
        let organ_transform = organ.get_transformation();
        turtle_transform = turtle_transform.mul_transform(organ_transform);
        organ.render_single(turtle_transform, organ_resources, commands);
    }
}