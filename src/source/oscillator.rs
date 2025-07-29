use crate::source::SourceTrait;
use std::f32::consts::{PI, TAU};

pub enum Waveform {
    Sine,
    Square,
    Triangle,
    PerfectSawtooth,
    SineSawtooth(u16),
}

pub struct Oscillator {
    waveform: Waveform,
    frequency: f32,
    amplitude: f32,
    modulation_amplitude: f32,
    modulation_frequency: f32,
}

impl SourceTrait for Oscillator {
    fn next_sample(&mut self, t: f32) -> f32 {
        let phase = (self.frequency * t
            + self.modulation_amplitude
                * self.frequency
                * (TAU * self.modulation_frequency * t).sin())
        .fract();

        let sample = match self.waveform {
            Waveform::Sine => (TAU * phase).sin(),
            Waveform::Square => {
                if phase < 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }

            // 4.0 * (phase - 0.5).abs() - 1.0 would suffice, but we shift the wave by 0.75 to have the same phase as the other functions
            Waveform::Triangle => 4.0 * ((phase + 0.75).fract() - 0.5).abs() - 1.0,

            // Again, shift the phase by 0.5
            Waveform::PerfectSawtooth => 2.0 * (phase + 0.5).fract() - 1.0,
            Waveform::SineSawtooth(n) => {
                let mut sawtooth = 0.0;
                for i in 1..=n {
                    sawtooth -= (TAU * i as f32 * (phase - 0.5)).sin() / (i as f32);
                }
                sawtooth * 2.0 / PI
            }
        };
        sample * self.amplitude
    }
}

impl Oscillator {
    pub fn new(
        waveform: Waveform,
        frequency: f32,
        amplitude: f32,
        modulation_amplitude: f32,
        modulation_frequency: f32,
    ) -> Self {
        Oscillator {
            waveform,
            frequency,
            amplitude,
            modulation_amplitude,
            modulation_frequency,
        }
    }
}
