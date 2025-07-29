mod audio_device;
mod envelope;
mod instrument;
mod source;
mod types;

use anyhow::Result;
use cpal::traits::StreamTrait;
use crossbeam::channel;
use envelope::EnvelopeADSR;
use instrument::{bell::Bell, InstrumentTrait};
use source::{
    noise::Noise,
    oscillator::{Oscillator, Waveform},
    SourceTrait,
};
use std::time::Instant;

fn main() -> Result<()> {
    let (s, r) = channel::unbounded();
    let (stream, sample_rate) = audio_device::stream_setup(r)?;
    stream.play()?;

    // let mut oscillator = Oscillator::new(Waveform::Sine, 0.004, 5.0);
    // let mut oscillator = Noise::new(0.03);
    // let mut envelope = EnvelopeADSR::new(0.3, 0.3, 0.4, 1.0, 0.6);
    let mut bell = Bell::default();

    let start_t = Instant::now();
    let mut sample_no: u64 = 0;
    // envelope.trigger_on(0.0);
    while start_t.elapsed().as_secs_f32() < 10.0 {
        while s.len() > 480 {}
        let t = sample_no as f64 / sample_rate.0 as f64;
        let sample =
            // 0.01 * oscillator.next_sample(440.0, t as f32) * envelope.get_amplitude(t as f32);
            0.1 * bell.make_sound(440.0, t as f32);
        s.send(types::Message(sample, sample))?;
        sample_no += 1;
    }
    Ok(())
}
