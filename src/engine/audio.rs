extern crate anyhow;
extern crate cpal;

use cpal::traits::StreamTrait;
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::InputCallbackInfo;
use macroquad::prelude::*;
use std::sync::mpsc::channel;

pub struct Audio {
    pub input_stream: cpal::Stream,
    pub channel_count: cpal::ChannelCount,
    pub audio_buffer: Vec<f32>,
    pub buffer: Image,
    pub audio_tex: Texture2D,
    pub other_buffer: Vec<f32>,
    pub recv: std::sync::mpsc::Receiver<Vec<f32>>,

    pub image_buffer: Image,
}

impl Audio {
    pub fn new() -> Self {
        let audio_buffer = Vec::new();
        let (send, recv) = channel();
        let (stream, channel_count) = create_input_stream(move |data, x| {
            let mut v = Vec::new();
            v.extend(data);
            send.send(v).unwrap();
        });
        let buffer = Image::gen_image_color(512, 1, BLACK);
        let audio_tex = Texture2D::from_image(&buffer);
        println!("{:?}", channel_count);
        Self {
            input_stream: stream,
            channel_count,
            audio_buffer,
            buffer: buffer,
            other_buffer: Vec::new(),
            recv,
            image_buffer: Image::gen_image_color(256, 256, BLACK),
            audio_tex: audio_tex,
        }
    }

    pub fn play(&mut self) {
        self.input_stream.play().unwrap();
    }

    pub fn pause(&mut self) {
        self.input_stream.pause().unwrap();
    }

    pub fn update(&mut self) {
        let mut i = 0;
        let mut done = false;
        // recv.try_recv(), do a whole thing match i guess?
        loop {
            match self.recv.try_recv() {
                Ok(data) => {
                    for sample in data {
                        if i < 512 * 4 {
                            self.buffer.bytes[i] = (127.0 * sample) as u8;
                            i += 1;
                        } else {
                            done = true;
                        }
                    }
                    if done {
                        break;
                    }
                }
                Err(e) => {
                    break;
                }
            }
        }
    }
}

fn test_stream(stream: &cpal::Stream) {
    use std::thread::sleep;
    use std::time::Duration;

    stream.play().unwrap();
    sleep(Duration::from_secs(1));
    stream.pause().unwrap();
}

fn create_input_stream<T>(func: T) -> (cpal::Stream, cpal::ChannelCount)
where
    T: FnMut(&[f32], &InputCallbackInfo) + std::marker::Send + 'static,
{
    let default_in = cpal::default_host().default_input_device().unwrap();
    let config = default_in.default_input_config().unwrap();

    let stream = default_in
        .build_input_stream::<f32, _, _>(&config.config(), func, |_e| {})
        .unwrap();

    // test_stream(&stream);
    (stream, config.channels())
}

pub fn print_audio_device_status() -> Result<(), anyhow::Error> {
    println!("Supported hosts:\n  {:?}", cpal::ALL_HOSTS);
    let available_hosts = cpal::available_hosts();
    println!("Available hosts:\n  {:?}", available_hosts);

    for host_id in available_hosts {
        println!("{}", host_id.name());
        let host = cpal::host_from_id(host_id)?;

        let default_in = host.default_input_device().map(|e| e.name().unwrap());
        let default_out = host.default_output_device().map(|e| e.name().unwrap());
        println!("  Default Input Device:\n    {:?}", default_in);
        println!("  Default Output Device:\n    {:?}", default_out);

        let devices = host.devices()?;
        println!("  Devices: ");
        for (device_index, device) in devices.enumerate() {
            println!("  {}. \"{}\"", device_index + 1, device.name()?);
        }
    }
    Ok(())
}
