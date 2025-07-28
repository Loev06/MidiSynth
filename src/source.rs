pub mod noise;
pub mod oscillator;

pub trait SourceTrait {
    fn next_sample(&mut self) -> f32;
}
