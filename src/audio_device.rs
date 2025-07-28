use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, HostTrait},
    FromSample, Sample, I24,
};
use crossbeam::channel::Receiver;

use crate::types::Message;

fn process_frame<SampleType>(
    output: &mut [SampleType],
    num_channels: usize,
    receiver: Receiver<Message>,
) -> Result<()>
where
    SampleType: Sample + FromSample<f32>,
{
    // if receiver.len() < output.len() / num_channels {
    //     eprintln!("Not enough messages in the channel, filling with zeros.");
    //     output.fill(SampleType::from_sample(0.0));
    //     return Ok(());
    // }

    for frame in output.chunks_mut(num_channels) {
        let message = receiver.try_recv().unwrap_or_else(|e| {
            eprintln!("Error receiving message: {}", e);
            Message::default()
        });

        frame[0] = SampleType::from_sample(message.0);
        if num_channels > 1 {
            frame[1] = SampleType::from_sample(message.1);
        }
    }
    Ok(())
}

fn make_stream<SampleType>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    receiver: Receiver<Message>,
) -> Result<cpal::Stream>
where
    SampleType: cpal::SizedSample + FromSample<f32>,
{
    let num_channels = config.channels as usize;
    let err_fn = |err| eprintln!("Error building output stream: {}", err);
    let stream = device.build_output_stream(
        config,
        move |output: &mut [SampleType], _: &cpal::OutputCallbackInfo| {
            process_frame(output, num_channels, receiver.clone()).unwrap_or_else(|e| {
                eprintln!("Error processing frame: {}", e);
            });
        },
        err_fn,
        None,
    )?;
    Ok(stream)
}

fn host_device_setup() -> Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig)> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::Error::msg("Default output device is not available"))?;
    println!("Output device: {}", device.name()?);

    let config = device.default_output_config()?;
    println!("Default output config: {:?}", config);

    Ok((host, device, config))
}

pub fn stream_setup(receiver: Receiver<Message>) -> Result<(cpal::Stream, cpal::SampleRate)> {
    let (_host, device, config) = host_device_setup()?;

    let sample_rate = config.sample_rate();
    let stream = match &config.sample_format() {
        cpal::SampleFormat::I8 => make_stream::<i8>(&device, &config.into(), receiver),
        cpal::SampleFormat::I16 => make_stream::<i16>(&device, &config.into(), receiver),
        cpal::SampleFormat::I24 => make_stream::<I24>(&device, &config.into(), receiver),
        cpal::SampleFormat::I32 => make_stream::<i32>(&device, &config.into(), receiver),
        cpal::SampleFormat::I64 => make_stream::<i64>(&device, &config.into(), receiver),
        cpal::SampleFormat::U8 => make_stream::<u8>(&device, &config.into(), receiver),
        cpal::SampleFormat::U16 => make_stream::<u16>(&device, &config.into(), receiver),
        cpal::SampleFormat::U32 => make_stream::<u32>(&device, &config.into(), receiver),
        cpal::SampleFormat::U64 => make_stream::<u64>(&device, &config.into(), receiver),
        cpal::SampleFormat::F32 => make_stream::<f32>(&device, &config.into(), receiver),
        cpal::SampleFormat::F64 => make_stream::<f64>(&device, &config.into(), receiver),
        sample_format => Err(anyhow::Error::msg(format!(
            "Unsupported sample format '{sample_format}'"
        ))),
    }?;

    Ok((stream, sample_rate))
}
