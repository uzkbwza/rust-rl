use specs::prelude::*;
use shrev::{EventChannel, Event, ReaderId};
use crate::systems::control;

pub struct DEBUG {
    pub input_reader: Option<ReaderId<control::KeyEvent>>,
    pub command_reader: Option<ReaderId<control::CommandEvent>>,
}

impl DEBUG{
    fn print_reader<T>(chanels: &EventChannel<T>, reader: &mut Option<ReaderId<T>>) 
    where T: Event + std::fmt::Debug {
        for event in chanels.read(reader.as_mut().unwrap()) {
                println!("{:?}", event )
        }
    }

    pub fn new() -> Self {
        DEBUG { 
            input_reader: None, 
            command_reader: None 
        }
    }
}

#[derive(SystemData)]
pub struct DebugSystemData<'a> {
    pub key_event_channels: Read<'a, EventChannel<control::KeyEvent>>,
    pub command_channels: Read<'a, EventChannel<control::CommandEvent>>,
}

impl<'a> System<'a> for DEBUG {
    type SystemData = DebugSystemData<'a>;

    fn run (&mut self, data: Self::SystemData) {
        Self::print_reader(&data.key_event_channels, &mut self.input_reader);
        Self::print_reader(&data.command_channels, &mut self.command_reader);
        }

    fn setup (&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        // finds the key event chanel inserted by the input system
        // and fetches it to create a reader as its own attribute
        self.input_reader = Some(world.
            fetch_mut::<EventChannel<control::KeyEvent>>()
            .register_reader());

        self.command_reader = Some(world.
            fetch_mut::<EventChannel<control::CommandEvent>>()
            .register_reader());
    }
}