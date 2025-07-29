pub mod noise;
pub mod oscillator;

pub trait SourceTrait {
    fn next_sample(&self, f: f32, t: f32) -> f32;
}
