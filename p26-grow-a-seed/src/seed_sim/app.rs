
use bevy::app::{App, Plugin, Startup, Update};
use crate::seed_sim::system_updates::{grow_seed, render_seed};
use crate::seed_sim::plant_bundle::PlantBundle;
use crate::seed_sim::plant_organs_resources::{OrganResources, StemBundle};
use crate::seed_sim::prelude::*;


pub struct PlantSimPlugin;
impl Plugin for PlantSimPlugin {
    fn build(&self, app: &mut App) {
        app
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
            .add_systems(Update, (grow_seed, render_seed).chain())
        ;
    }
}

fn add_rendering(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn add_seeds(mut commands: Commands) {
    let mut rng = SmallRng::from_entropy();
    for _ in 0..SEED_N {
        commands.spawn(PlantBundle::new(&mut rng, SPACE_SIZE as f32));
    }
    println!("Added {} seeds", SEED_N);
}