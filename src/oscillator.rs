use std::f32::consts::TAU;

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
    phase: f32,
    sample_rate: u32,
}

impl Oscillator {
    pub fn new(
        waveform: Waveform,
        frequency: f32,
        amplitude: f32,
        phase: f32,
        sample_rate: u32,
    ) -> Self {
        Oscillator {
            waveform,
            frequency,
            amplitude,
            phase,
            sample_rate,
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let sample = match self.waveform {
            Waveform::Sine => self.amplitude * (self.phase * TAU).sin(),
            Waveform::Square => {
                if self.phase < 0.5 {
                    self.amplitude
                } else {
                    -self.amplitude
                }
            }
            Waveform::Triangle => 2.0 * self.amplitude * (self.phase - 0.5).abs() - self.amplitude,
            Waveform::PerfectSawtooth => self.amplitude * (2.0 * self.phase - 1.0),
            Waveform::SineSawtooth(n) => {
                let mut sawtooth = 0.0;
                for i in 1..=n {
                    sawtooth -= (i as f32 * self.phase * TAU).sin() / (i as f32);
                }
                sawtooth * self.amplitude
            }
        };

        self.phase += self.frequency / self.sample_rate as f32;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        sample
    }
}
