use midir::{MidiInput, Ignore};
use macroquad::*;

// Custom ECS event for MIDI events
#[derive(Clone, Debug)]
struct MidiEvent {
    // MIDI message data
    message: Vec<u8>,
}

// Custom system to listen for MIDI events and dispatch the corresponding ECS event
struct MidiSystem {
    // MIDI input
    midi_in: MidiInput,
    // Port name
    port_name: String,
}

impl MidiSystem {
    fn new() -> Self {
        let mut midi_in = MidiInput::new("My MIDI Input").unwrap();
        let port_name = midi_in.ports()[0].name.clone();
        let mut port = midi_in.port(&port_name, "midir reading input").unwrap();

        // Ignore all messages
        port.ignore(Ignore::All);
        // Listen for note on and note off messages
        port.listen(|_, message, _| {
            let message = message.to_vec();
            let event = MidiEvent { message };
            // Dispatch the ECS event
            world::dispatch_event(event);
        });

        MidiSystem { midi_in, port_name }
    }
}

impl System for MidiSystem {
    fn run(&mut self) {
        self.midi_in.process_events().unwrap();
    }
}

#[macroquad::main("MIDI ECS Events")]
async fn main() {
    // Add the custom MidiSystem to the world
    world::add_system(MidiSystem::new());

    loop {
        clear_background(BLACK);

        // Process events
        world::process_events();

        end_frame();
    }
}
