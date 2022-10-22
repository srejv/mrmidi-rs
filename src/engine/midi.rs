use macroquad::prelude::*;
use midir::{Ignore, MidiInput};
use midly::num::u4;
use midly::num::u7;
use midly::{live::LiveEvent, MidiMessage};
use std::io::Write;
use std::sync::mpsc::channel;

fn u4_to_usize(x: &u4) -> usize {
    x.as_int().into()
}

fn u7_to_usize(x: &u7) -> usize {
    x.as_int().into()
}

fn u7_to_u8(x: &u7) -> u8 {
    x.as_int()
}

fn channel_key_to_index(channel: &u4, key: &u7, width: usize, offset: usize) -> usize {
    u7_to_usize(key) + (u4_to_usize(channel) + offset) * width
}

pub struct Midi {
    pub buffer: Image,
    pub tex: Texture2D,
    pub recv: Option<std::sync::mpsc::Receiver<Vec<u8>>>,
    input_connection: Option<midir::MidiInputConnection<()>>,
}

impl Midi {
    pub fn new() -> Self {
        let buffer = Image::gen_image_color(32, 32, WHITE);
        let tex = Texture2D::from_image(&buffer);

        Self {
            buffer: buffer,
            tex: tex,
            recv: None,
            input_connection: None,
        }
    }

    pub fn start(&mut self) {
        let mut midi_in = MidiInput::new("midir reading input").unwrap();
        midi_in.ignore(Ignore::None);
        let in_ports = midi_in.ports();
        let in_port = match in_ports.len() {
            0 => return,
            1 => {
                println!(
                    "Choosing the only available input port: {}",
                    midi_in.port_name(&in_ports[0]).unwrap()
                );
                &in_ports[0]
            }
            _ => {
                println!("\nAvailable input ports:");
                for (i, p) in in_ports.iter().enumerate() {
                    println!("{}: {}", i, midi_in.port_name(p).unwrap());
                }
                println!("Please select input port: ");
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let x = input.trim().parse::<usize>().unwrap();
                let inp = in_ports
                    .get(x)
                    .ok_or("invalid input port selected")
                    .unwrap();
                inp
            }
        };
        let (send, recv) = std::sync::mpsc::channel();
        self.input_connection = Some(
            midi_in
                .connect(
                    in_port,
                    "midir-read-input",
                    move |stamp, message, _| {
                        // println!("{}: {:?} (len = {})", stamp, message, message.len());
                        let mut msg: Vec<u8> = Vec::new();
                        msg.extend_from_slice(message);
                        send.send(msg);
                    },
                    (),
                )
                .unwrap(),
        );

        self.recv = Some(recv);
    }

    pub fn update(&mut self) {
        if let Some(r) = &self.recv {
            match r.try_recv() {
                Ok(data) => {
                    // What is this? Parsing?
                    let event = LiveEvent::parse(&data).unwrap();
                    match event {
                        LiveEvent::Midi { channel, message } => match message {
                            MidiMessage::NoteOn { key, vel } => {
                                let width: usize = self.buffer.width() * 4;
                                let index = channel_key_to_index(&channel, &key, width, 0);
                                self.buffer.bytes[index] = u7_to_u8(&vel) * 2;
                            }

                            MidiMessage::NoteOff { key, vel } => {
                                let width: usize = self.buffer.width() * 4;
                                let index = channel_key_to_index(&channel, &key, width, 0);
                                self.buffer.bytes[index] = 0;
                            }

                            MidiMessage::Controller { controller, value } => {
                                let width: usize = self.buffer.width() * 4;
                                let index = channel_key_to_index(&channel, &controller, width, 16);
                                self.buffer.bytes[index] = value.as_int() * 2;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

use std::error::Error;

pub fn test_midi() -> Result<(), Box<dyn Error>> {
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );
            &in_ports[0]
        }
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            println!("Please select input port: ");
            std::io::stdout().flush()?;
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let key = input.trim().parse::<usize>()?;
            in_ports.get(key).unwrap()
        }
    };

    println!("Opening connection!");

    let mut input: String = "".to_owned();

    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |stamp, message, _| {
            println!("{}: {:?} (len = {})", stamp, message, message.len());
        },
        (),
    )?;

    input.clear();
    std::io::stdin().read_line(&mut input)?;

    println!("Closing connection!");
    Ok(())
}
