use specs::prelude::*;
use shrev::{EventChannel, Event, ReaderId};
use crate::systems::input;
use crate::systems::movement;
use crate::components::PrintDebug;

pub struct DEBUG {
    pub input_reader: Option<ReaderId<input::KeyEvent>>,
    pub move_event_reader: Option<ReaderId<movement::MoveEvent>>,
}

impl DEBUG{
    fn _print_reader<T>(channel: &EventChannel<T>, reader: &mut Option<ReaderId<T>>)  
    where T: Event + std::fmt::Debug {
        for event in channel.read(reader.as_mut().unwrap()) {

            println!("{:?}", event )
        }
    }

    pub fn new() -> Self {
        DEBUG { 
            input_reader: None, 
            move_event_reader: None 
        }
    }
}

#[derive(SystemData)]
pub struct DebugSystemData<'a> {
    pub entities: Entities<'a>,
    pub debug_printables: ReadStorage<'a, PrintDebug>,
    pub key_event_channels: Read<'a, EventChannel<input::KeyEvent>>,
    pub move_event_channels: Read<'a, EventChannel<movement::MoveEvent>>,
}

impl<'a> System<'a> for DEBUG {
    type SystemData = DebugSystemData<'a>;

    fn run (&mut self, _data: Self::SystemData) {
        // Self::print_reader(&data.key_event_channels, &mut self.input_reader);
        // Self::print_reader(&data.move_event_channels, &mut self.move_event_reader);
        }

    fn setup (&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        
        // finds the key event chanel inserted by the input system
        // and fetches it to create a reader as its own attribute
        self.input_reader = Some(world.
            fetch_mut::<EventChannel<input::KeyEvent>>()
            .register_reader());

        self.move_event_reader = Some(world.
            fetch_mut::<EventChannel<movement::MoveEvent>>()
            .register_reader());
    }
}