
use bevy::app::{App, Plugin, Startup, Update};
use crate::seed_sim::organ_parent_transform_propigation::propagate_custom_transforms;
use crate::seed_sim::parent_retargeting::{OrganParentRetargetingResources, parent_retargeting, print_parent_relationships, PrintTimer, update_spawn_status_end_frame};
use crate::seed_sim::system_updates::{organ_production};
use crate::seed_sim::plant_bundle::{OrganBundle};
use crate::seed_sim::plant_organs_resources::{OrganResources, StemBundle};
use crate::seed_sim::prelude::*;


pub struct PlantSimPlugin;


impl Plugin for PlantSimPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(OrganParentRetargetingResources{ parent_retargets: None })
            .insert_resource(PrintTimer(Timer::from_seconds(0.9, TimerMode::Repeating)))
            .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
            .insert_resource(OrganResources {
                stem_bundle: StemBundle {
                    sprite_bundle: SpriteBundle {
                        transform: Transform::from_scale(Vec3::new(0.1, 0.1, 1.0)),
                        ..Default::default()
                    },
                },
            })
            .add_systems(Startup, (add_rendering, add_seeds))
            .add_systems(Update, organ_production)
            .add_systems(Update, parent_retargeting
                .after(organ_production)
                .before(update_spawn_status_end_frame)
            )
            .add_systems(Update, update_spawn_status_end_frame)
            .add_systems(Update, propagate_custom_transforms)
            .add_systems(Update, print_parent_relationships)
        ;
    }
}

fn add_rendering(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn add_seeds(mut commands: Commands) {
    let mut rng = SmallRng::from_entropy();
    for _ in 0..SEED_N {
        commands.spawn(OrganBundle::new(&mut rng, SPACE_SIZE as f32));
    }
    println!("Added {} seeds", SEED_N);
}