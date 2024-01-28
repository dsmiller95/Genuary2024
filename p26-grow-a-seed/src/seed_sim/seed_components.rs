use bevy::prelude::Component;
use super::prelude::*;

#[derive(Component)]
pub struct SeedTimer(pub Timer);

#[derive(Component, Debug)]
pub struct EntityOrgan{
    pub organ: Organ,
}

#[derive(Component, Debug, Clone)]
pub struct OrganLifespan{pub remaining: Timer}

#[derive(Component, Debug)]
pub struct OrganRelations{
    pub parent: Option<Entity>
}

#[derive(Component, Debug)]
pub struct SpawnStatus(pub SpawnedTime);
#[derive(Debug)]
pub enum SpawnedTime{
    ThisFrame,
    OlderFrame,
}

#[derive(Debug, Clone)]
pub enum Organ{
    Stem(Stem),
    Crook{angle: f32},
    Leaf,
    Flower,
    Fruit,
    Root{rotation: f32},
    Origin,
    Seed,
}

#[derive(Debug, Clone)]
pub struct Stem{
    /// Length of the stem in pixels
    /// max 10
    pub length: f32,
}
