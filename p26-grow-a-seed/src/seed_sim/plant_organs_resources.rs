use crate::seed_sim::prelude::*;

#[derive(Resource, Clone)]
pub struct OrganResources {
    pub stem_bundle : StemBundle,
}

#[derive(Bundle, Clone)]
pub struct StemBundle {
    pub sprite_bundle: SpriteBundle,
}



