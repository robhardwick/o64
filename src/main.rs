mod envelope;
mod error;
mod filter;
mod operator;
mod synth;
mod util;

use std::env::args;
use std::time::{SystemTime, UNIX_EPOCH};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat};

use error::Error;
use synth::Synth;

const DEFAULT_SEED: u64 = 1;
const DEFAULT_TEMPO: u64 = 60;

fn main() -> Result<(), Error> {
    let seed = args().nth(1).map_or_else(
        || {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_or(DEFAULT_SEED, |duration| duration.as_secs())
        },
        |arg| arg.parse::<u64>().unwrap_or(DEFAULT_SEED),
    );
    println!("SEED: {}", seed);

    let tempo = args().nth(2).map_or(DEFAULT_TEMPO, |arg| {
        arg.parse::<u64>().unwrap_or(DEFAULT_TEMPO)
    });
    println!("TEMPO: {}", tempo);

    let host = cpal::default_host();
    let device = host.default_output_device().ok_or(Error::NoOutputDevice)?;
    let mut supported_configs_range = device.supported_output_configs()?;
    let supported_config = supported_configs_range
        .next()
        .ok_or(Error::NoConfig)?
        .with_max_sample_rate();

    let sample_format = supported_config.sample_format();
    let config = supported_config.into();

    match sample_format {
        SampleFormat::F32 => run::<f32>(seed, tempo, device, config),
        SampleFormat::I16 => run::<i16>(seed, tempo, device, config),
        SampleFormat::U16 => run::<u16>(seed, tempo, device, config),
    }
}

fn run<T>(
    seed: u64,
    tempo: u64,
    device: cpal::Device,
    config: cpal::StreamConfig,
) -> Result<(), Error>
where
    T: cpal::Sample,
{
    let mut synth = Synth::new(seed, tempo, config.sample_rate.0 as f32);
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            for (sample, value) in data.iter_mut().zip(&mut synth) {
                *sample = Sample::from(&value);
            }
        },
        |err| eprintln!("stream error: {}", err),
    )?;

    stream.play()?;
    std::thread::park();

    Ok(())
}
