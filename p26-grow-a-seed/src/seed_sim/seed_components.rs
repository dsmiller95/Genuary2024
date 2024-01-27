use bevy::prelude::Component;
use super::prelude::*;

#[derive(Component)]
pub struct SeedTimer(pub Timer);

#[derive(Component, Debug)]
pub struct Seed{
    pub organs: Vec<Organ>,
    pub steps: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Organ{
    Stem(Stem),
    Leaf,
    Flower,
    Fruit,
    Root,
}

#[derive(Debug, Clone, Copy)]
pub struct Stem{
    /// Length of the stem in pixels
    /// max 10
    pub length: f32,
}
