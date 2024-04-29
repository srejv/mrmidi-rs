use macroquad::*;

// Custom ECS event for deleting entities
#[derive(Clone, Debug)]
struct DeleteEvent {
    // Entity to delete
    entity: Entity,
}

// Custom system to listen for MIDI events and create entities
struct MidiSystem {
    // Entities with a selected component
    selected_entities: Vec<Entity>,
}

impl<'a> System<'a> for MidiSystem {
    type SystemData = (Read<'a, Events<MidiEvent>>, Write<'a, Events<DeleteEvent>>);

    fn run(&mut self, (midi_events, mut delete_events): Self::SystemData) {
        for event in midi_events.iter() {
            let message = event.message;
            // Check for MIDI note C-5 message
            if message.len() == 3 && message[0] == 144 && message[1] == 60 {
                // Create a new entity with a selected component
                let entity = world::create_entity().with(Selected).build();
                // Add the entity to the list of selected entities
                self.selected_entities.push(entity);
            }
        }
        // Remove any entities with a selected component from the world
        for entity in self.selected_entities.drain(..) {
            delete_events.single_write(DeleteEvent { entity });
        }
    }
}

struct MidiSystem2;

impl<'a> System<'a> for MidiSystem2 {
    type SystemData = Read<'a, Events<MidiEvent>>;

    fn run(&mut self, midi_events: Self::SystemData) {
        for event in midi_events.iter() {
            let message = event.message;
            // Check for MIDI note D-5 message
            if message.len() == 3 && message[0] == 144 && message[1] == 62 {
                // Create a new entity
                let _entity = world::create_entity().build();
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Selected;

// Custom system to listen for delete events and remove entities from the world
struct DeleteSystem;

impl<'a> System<'a> for DeleteSystem {
    type SystemData = Read<'a, Events<DeleteEvent>>;

    fn run(&mut self, delete_events: Self::SystemData) {
        for event in delete_events.iter() {
            let entity = event.entity;
            world::delete_entity(entity);
        }
    }
}

#[macroquad::main("MIDI ECS Events")]
async fn main() {
    // Add the custom MidiSystem and DeleteSystem to the world
    world::add_system(MidiSystem { selected_entities: Vec::new() });
    world::add_system(DeleteSystem);

    loop {
        clear_background(BLACK);

        // Process events
        world::process
