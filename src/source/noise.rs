use crate::source::SourceTrait;
use rand::Rng;

pub struct Noise {
    amplitude: f32,
}

impl SourceTrait for Noise {
    fn next_sample(&mut self, t: f32) -> f32 {
        let mut rng = rand::rng();
        rng.random_range(-1.0..=1.0) * self.amplitude
    }
}

impl Noise {
    pub fn new(amplitude: f32) -> Self {
        Noise { amplitude }
    }
}
