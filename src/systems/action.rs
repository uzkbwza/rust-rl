use specs::prelude::*;
use crate::command::{Command, CommandEvent};
use crate::systems::movement::{Dir};
use crate::components::flags::requests::*;
use shrev::{EventChannel, ReaderId};

pub struct ActionHandler {
    command_event_reader: Option<ReaderId<CommandEvent>>
}

impl ActionHandler {
    pub fn new() -> Self {
        ActionHandler {
            command_event_reader: None
        }
    }
}

#[derive(SystemData)]
pub struct ActionHandlerSystemData<'a> {
        move_requests: WriteStorage<'a, MoveRequest>,
        attack_requests: WriteStorage<'a, AttackRequest>,

        // read event channels
        command_event_channel: Read<'a, EventChannel<CommandEvent>>,
}

impl<'a> System<'a> for ActionHandler {
    type SystemData = ActionHandlerSystemData<'a>;
    
    fn run(&mut self, mut data: Self::SystemData) {
        let command_events = data.command_event_channel.read(self.command_event_reader.as_mut().unwrap());
        for command_event in command_events {
            let entity = command_event.entity;
            match command_event.command {
                Command::Move(dir) => {
                    let (dx, dy) = Dir::dir_to_pos(dir);
                    let move_request = MoveRequest::new(dx, dy);
                    data.move_requests.insert(entity, move_request);
                },
                
                Command::Attack(dir) => {
                    let attack_request = AttackRequest::new(dir);
                    data.attack_requests.insert(entity, attack_request);
                }
                _ => (),
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.command_event_reader = Some(world.
            fetch_mut::<EventChannel<CommandEvent>>()
            .register_reader());
    }
}
