use crate::source::SourceTrait;
use rand::Rng;

pub struct Noise;

impl SourceTrait for Noise {
    fn next_sample(&self, _f: f32, _t: f32) -> f32 {
        let mut rng = rand::rng();
        rng.random_range(-1.0..=1.0)
    }
}
