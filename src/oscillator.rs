use std::f32::consts::TAU;

pub struct Oscillator {
    frequency: f32,
    amplitude: f32,
    phase: f32,
    sample_rate: u32,
}

impl Oscillator {
    pub fn new(frequency: f32, amplitude: f32, phase: f32, sample_rate: u32) -> Self {
        Oscillator {
            frequency,
            amplitude,
            phase,
            sample_rate,
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let sample = self.amplitude * (self.phase * TAU).sin();
        self.phase += self.frequency / self.sample_rate as f32;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        sample
    }
}
