
use bevy::app::{App, Plugin, Startup, Update};
use crate::seed_sim::grow_system::grow_seed;
use crate::seed_sim::plant_bundle::PlantBundle;
use crate::seed_sim::prelude::*;


pub struct PlantSimPlugin;
impl Plugin for PlantSimPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
            .add_systems(Startup, (add_rendering, add_seeds))
            .add_systems(Update, grow_seed)
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