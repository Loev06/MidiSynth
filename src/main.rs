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
use std::time::Instant;

fn main() -> Result<()> {
    let (s, r) = channel::unbounded();
    let (stream, sample_rate) = audio_device::stream_setup(r)?;
    stream.play()?;

    let mut oscillator = Oscillator::new(Waveform::Triangle, 440.0, 0.02, 0.0002, 4.0);
    // let mut oscillator = Noise::new(0.03);
    let mut envelope = EnvelopeADSR::new(0.3, 0.3, 0.4, 1.0, 0.6);

    // for i in 0..(sample_rate.0 * 2) {
    //     let t = i as f32 / sample_rate.0 as f32;

    //     match i {
    //         0 => envelope.trigger_on(t),
    //         50000 => envelope.trigger_off(t),
    //         _ => (),
    //     }

    //     let sample = oscillator.next_sample() * envelope.get_amplitude(t);
    //     s.send(types::Message(sample, sample))?;
    // }

    let start_t = Instant::now();
    let mut sample_no: u64 = 0;
    envelope.trigger_on(0.0);
    while start_t.elapsed().as_secs_f32() < 10.0 {
        // while s.len() > 480 {}
        let current_t = start_t.elapsed().as_secs_f32();
        for i in 0..480 {
            let t = sample_no as f64 / sample_rate.0 as f64;
            let t2 = start_t.elapsed().as_secs_f32();
            let sample = oscillator.next_sample(t as f32) * envelope.get_amplitude(t as f32);
            s.send(types::Message(sample, sample))?;
            sample_no += 1;
        }
    }
    let left = s.len() as u32 / sample_rate.0;
    println!("{}", left);
    std::thread::sleep(std::time::Duration::from_millis(left as u64 * 1000u64));
    Ok(())
}
