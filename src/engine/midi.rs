use macroquad::prelude::*;
use midir::{Ignore, MidiInput};
use std::io::Write;
use std::sync::mpsc::channel;

pub struct Midi {
    pub buffer: Image,
    pub recv: Option<std::sync::mpsc::Receiver<Vec<u8>>>,
}

impl Midi {
    pub fn new() -> Self {
        Self {
            buffer: Image::gen_image_color(32, 32, BLACK),
            recv: None,
        }
    }
}

pub fn midi_start(midi: &mut Midi) {
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
    let _conn_in = midi_in
        .connect(
            in_port,
            "midir-read-input",
            move |stamp, message, _| {
                println!("{}: {:?} (len = {})", stamp, message, message.len());
                let mut msg: Vec<u8> = Vec::new();
                msg.extend_from_slice(message);
                send.send(msg);
            },
            (),
        )
        .unwrap();

    midi.recv = Some(recv);
}

use midly::num::u4;
use midly::num::u7;
use midly::{live::LiveEvent, MidiMessage};

fn u4_to_usize(x: u4) -> usize {
    x.as_int().into()
}

fn u7_to_usize(x: u7) -> usize {
    x.as_int().into()
}

fn u7_to_u8(x: u7) -> u8 {
    x.as_int()
}

pub fn midi_update(midi: &mut Midi) {
    // Receive and parse midi updates

    if let Some(r) = &midi.recv {
        match r.try_recv() {
            Ok(data) => {
                // What is this? Parsing?
                let event = LiveEvent::parse(&data).unwrap();
                match event {
                    LiveEvent::Midi { channel, message } => match message {
                        MidiMessage::NoteOn { key, vel } => {
                            println!("hit note {} on channel {}", key, channel);
                            // Set byte on channel.
                            let ch = 0 + u4_to_usize(channel);
                            let width: usize = midi.buffer.width();
                            let index = u7_to_usize(key) + ch * width;
                            midi.buffer.bytes[index] = u7_to_u8(vel);
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
