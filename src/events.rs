
enum Event {
    CreateEntity,
    SelectEntity(entity),
    AddComponent(entity, component)
    RemoveEntity(entity),
    SetVisible(entity, visible),
    EditComponent(entity, component, param, value),
    CreateEntityWith(components),
}

pub fn create_events_from_midi() -> Vec<Event> {
    Vec::new()
}

pub fn process_events(events: &[Event]) {
    for event in events {
        match event {
            Event::CreateEntity => {
                // world.create_entity();
                println!("Create Entity");
            },
            Event::EditEntity => {
                // world.get_entity().set_component(component?)
                println!("Edit Entity");
            },
            Event::RemoveEntity => {
                println!("Remove Entity");
            }
            Event::ChangeComponentOfEntity => {
                println!("Change Component");
            }
            _ => {
                println!("Not implemented");
            }
        }
    }
}

fn main() {
    let events = [
        Event::CreateEntity,
        Event::CreateEntity,
        Event::CreateEntity,
    ];
    // World? App state?
    // 
}