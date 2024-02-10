
pub const SEED_N: usize = 10;
pub const MAXIMUM_ENTITY_COUNT: usize = 500_000;

pub const INITIAL_LIFESPAN_SECONDS: f32 = 30.0;
pub const LIFESPAN_JITTER: f32 = 0.25;

pub const SPACE_SIZE: usize = 500;

pub const GROWTH_DELAY: f32 = 0.1;
pub const GROWTH_JITTER_FACTOR: f32 = 0.5;

pub const EPSILON: f32 = 0.0000001;

pub const MAX_STEM_LENGTH: f32 = 3.0;
pub const STEM_GROWTH_PER_STEP: f32 = 0.1;
pub const SEGMENT_LEN: f32 = 1.0;


pub struct OrganGenerationConsts {
    pub max_stem_length: f32,
    pub stem_growth_per_step: f32,
    pub segment_len: f32,
}

impl Default for OrganGenerationConsts {
    fn default() -> Self {
        OrganGenerationConsts {
            max_stem_length: MAX_STEM_LENGTH,
            stem_growth_per_step: STEM_GROWTH_PER_STEP,
            segment_len: SEGMENT_LEN,
        }
    }
}