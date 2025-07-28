mod audio_device;
mod source;
mod types;

use anyhow::Result;
use cpal::traits::StreamTrait;
use crossbeam::channel;
use source::{
    noise::Noise,
    oscillator::{Oscillator, Waveform},
    SourceTrait,
};

fn main() -> Result<()> {
    let (s, r) = channel::unbounded();
    let (stream, sample_rate) = audio_device::stream_setup(r)?;
    stream.play()?;

    let mut oscillator =
        // Oscillator::new(Waveform::SineSawtooth(10), 110.0, 0.03, 0.0, sample_rate.0);
        Noise::new(0.03);
    for _ in 0..(sample_rate.0 * 2) {
        let sample = oscillator.next_sample();
        s.send(types::Message(sample, sample))?;
    }

    std::thread::sleep(std::time::Duration::from_millis(2000));
    Ok(())
}
