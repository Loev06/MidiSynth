pub mod noise;
pub mod oscillator;

pub trait SourceTrait {
    fn next_sample(&mut self, t: f32) -> f32;
}
