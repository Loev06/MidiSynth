mod audio_device;
mod envelope;
mod source;
mod types;

use anyhow::Result;
use cpal::traits::StreamTrait;
use crossbeam::channel;
use envelope::EnvelopeADSR;
use source::{
    noise::Noise,
    oscillator::{Oscillator, Waveform},
    SourceTrait,
};

fn main() -> Result<()> {
    let (s, r) = channel::unbounded();
    let (stream, sample_rate) = audio_device::stream_setup(r)?;
    stream.play()?;

    let mut oscillator = Oscillator::new(Waveform::Square, 440.0, 0.02, 0.0, sample_rate.0);
    // Noise::new(0.03);
    let mut envelope = EnvelopeADSR::new(0.3, 0.3, 0.4, 1.0, 0.6);

    for i in 0..(sample_rate.0 * 2) {
        let t = i as f32 / sample_rate.0 as f32;

        match i {
            0 => envelope.trigger_on(t),
            50000 => envelope.trigger_off(t),
            _ => (),
        }

        let sample = oscillator.next_sample() * envelope.get_amplitude(t);
        s.send(types::Message(sample, sample))?;
    }

    std::thread::sleep(std::time::Duration::from_millis(2000));
    Ok(())
}
