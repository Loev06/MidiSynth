mod audio_device;
mod oscillator;
mod types;

use anyhow::Result;
use cpal::traits::StreamTrait;
use crossbeam::channel::unbounded;

fn main() -> Result<()> {
    let (s, r) = unbounded();
    let (stream, sample_rate) = audio_device::stream_setup(r)?;
    stream.play()?;

    let mut oscillator = oscillator::Oscillator::new(440.0, 0.1, 0.0, sample_rate.0);
    for _ in 0..(sample_rate.0 * 2) {
        let sample = oscillator.next_sample();
        s.send(types::Message(sample, sample))?;
    }

    std::thread::sleep(std::time::Duration::from_millis(2000));
    Ok(())
}
