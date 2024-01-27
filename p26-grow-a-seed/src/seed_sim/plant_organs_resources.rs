use crate::seed_sim::prelude::*;

#[derive(Resource)]
pub struct StemResource{
    pub sprite : Sprite,
    pub texture: Handle<Image>,
}
