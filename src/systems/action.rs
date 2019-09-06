use specs::prelude::*;
use crate::systems::control::{Command, CommandEvent};
use crate::systems::movement::{MoveCommand, dir_to_pos};
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
        _entities: Entities<'a>,

        // read event channels
        command_event_channel: Read<'a, EventChannel<CommandEvent>>,

        // write event channels
        move_command_channel: Write<'a, EventChannel<MoveCommand>>,
}

impl<'a> System<'a> for ActionHandler {
    type SystemData = ActionHandlerSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let commands = data.command_event_channel
            .read(self.command_event_reader
            .as_mut()
            .unwrap());
        for command in commands {
            match command.command {
                Command::Move(dir) => { 
                    let (x, y) = dir_to_pos(dir);
                    data.move_command_channel
                        .single_write(MoveCommand::new(command.entity, x, y)); },
                _ => (),
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.command_event_reader = Some(world.
            fetch_mut::<EventChannel<CommandEvent>>()
            .register_reader());

        let move_command_channel: EventChannel<MoveCommand> = EventChannel::new();
        world.insert(move_command_channel);
    }
}
