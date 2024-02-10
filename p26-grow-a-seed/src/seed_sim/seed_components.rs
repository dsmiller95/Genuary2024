use bevy::prelude::Component;
use float_cmp::ApproxEq;
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

#[derive(Debug, Clone, PartialEq)]
pub enum Organ{
    Stem(Stem),
    //EventualBranch{ time_till_branch: f32},
    Crook{angle: f32},
    Leaf,
    Flower,
    Fruit,
    StemSeg,
    Root{rotation: f32},
    Origin,
    Seed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stem{
    pub partial_length: f32,
    pub generated_segments: u8
}

impl Default for Stem {
    fn default() -> Self {
        Stem {
            partial_length: 0.0,
            generated_segments: 0,
        }
    }
}

impl ApproxEq for &Stem {
    type Margin = <f32 as ApproxEq>::Margin;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.partial_length.approx_eq(other.partial_length, margin)
            && self.generated_segments.eq(&other.generated_segments)
    }
}

impl ApproxEq for &Organ {
    type Margin = <f32 as ApproxEq>::Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        match (self, other) {
            (Organ::Stem(a), Organ::Stem(b)) => a.approx_eq(b, margin),
            (Organ::Crook{angle: a}, Organ::Crook{angle: b}) => a.approx_eq(*b, margin),
            (Organ::Leaf, Organ::Leaf) => true,
            (Organ::Flower, Organ::Flower) => true,
            (Organ::Fruit, Organ::Fruit) => true,
            (Organ::StemSeg, Organ::StemSeg) => true,
            (Organ::Root{rotation: a}, Organ::Root{rotation: b}) => a.approx_eq(*b, margin),
            (Organ::Origin, Organ::Origin) => true,
            (Organ::Seed, Organ::Seed) => true,
            //(Organ::EventualBranch{time_till_branch: a}, Organ::EventualBranch{time_till_branch: b}) => a.approx_eq(*b, margin),
            _ => false
        }
    }
}