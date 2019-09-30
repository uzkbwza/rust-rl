use tcod::console::*;
use tcod::input::{KeyCode, KeyPressFlags};
use tcod::input::Key as TcodKey;

use specs::prelude::*;
use shrev::{EventChannel, Event};

use crate::command::{Command, CommandEvent};
use crate::map::{EntityMap, View};
use crate::components::{PlayerControl, MyTurn, Position};
use crate::systems::movement::{Dir};
use std::num;
use rltk::VirtualKeyCode;

#[derive(Debug)]
pub struct Input {
    key_queue: Vec<VirtualKeyCode>,
    key_reader: Option<ReaderId<VirtualKeyCode>>,
}

pub trait KeyInterface {

}

impl Input {
    pub fn new() -> Self {
        Input {
            key_queue: Vec::new(),
            key_reader: None,
        }
    }

    fn get_command_from_key(key: VirtualKeyCode) -> Option<Command> {
        match key {
            // global commands
            VirtualKeyCode::Escape => Some(Command::EndGame),

            // actor commands
            VirtualKeyCode::H | VirtualKeyCode::Numpad4 => Some(Command::Move(Dir::W)),
            VirtualKeyCode::J | VirtualKeyCode::Numpad2 => Some(Command::Move(Dir::S)),
            VirtualKeyCode::K | VirtualKeyCode::Numpad8 => Some(Command::Move(Dir::N)),
            VirtualKeyCode::L | VirtualKeyCode::Numpad6 => Some(Command::Move(Dir::E)),
            VirtualKeyCode::Y | VirtualKeyCode::Numpad7 => Some(Command::Move(Dir::NW)),
            VirtualKeyCode::U | VirtualKeyCode::Numpad9 => Some(Command::Move(Dir::NE)),
            VirtualKeyCode::B | VirtualKeyCode::Numpad1 => Some(Command::Move(Dir::SW)),
            VirtualKeyCode::N | VirtualKeyCode::Numpad3 => Some(Command::Move(Dir::SE)),
            VirtualKeyCode::Period => Some(Command::Move(Dir::Nowhere)),
            _ => None
        }
    }
}

#[derive(SystemData)]
pub struct InputSystemData<'a> {
    pub entities:   Entities<'a>,
    pub entity_map: ReadExpect<'a, EntityMap>,
    pub view: ReadExpect<'a, View>,
    pub players:    ReadStorage<'a, PlayerControl>,
    pub positions: ReadStorage<'a, Position>,
    pub my_turns:   WriteStorage<'a, MyTurn>,
    pub world_updater:          Read<'a, LazyUpdate>,
    pub world_resources: WriteExpect<'a, crate::WorldResources>,
    pub key_channel:      Read<'a, EventChannel<VirtualKeyCode>>,
    pub command_event_channel:  Write<'a, EventChannel<CommandEvent>>,
}

impl<'a> System<'a> for Input {
    type SystemData = InputSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {

        let keys = data.key_channel.read(self.key_reader.as_mut().unwrap());
        for key in keys {
            self.key_queue.push(*key);
        }
        if self.key_queue.is_empty() { return }

        let command = Self::get_command_from_key(self.key_queue.pop().unwrap());
        // meta commands
        match command {
            Some(Command::ToggleRealTime) => data.world_resources.real_time = !data.world_resources.real_time,
            _ => (),
        }

        for (ent, _player, _my_turn) in (&data.entities, &data.players, &mut data.my_turns).join() {
            match command {

                // player commands
                Some(Command::Move(dir)) => {
                    // attach action component to player entity
                    let mut command_event = CommandEvent::new(command.unwrap(), ent);

                    if let Some(pos) = data.positions.get(ent) {
                        let fov_map = data.view.map.lock().unwrap();
                        let dpos = Dir::dir_to_pos(dir);
                        let dest = (
                            match dpos.0 + pos.x {
                                x if x >= fov_map.size().0 => fov_map.size().0 - 1,
                                x if x < 0 => 0,
                                _ => dpos.0 + pos.x
                            },
                            match dpos.1 + pos.y {
                                y if y >= fov_map.size().1 => fov_map.size().1 - 1,
                                y if y < 0 => 0,
                                _ => dpos.1 + pos.y
                            },
                        );

                        if !fov_map.is_walkable(dest.0, dest.1) && dir != Dir::Nowhere {

                            // attack enemy instead if closeby
                            if let Some(target_entity) = data.entity_map.actors.get(dest.0, dest.1) {
                                command_event = CommandEvent::new(Command::Attack(dir), ent);

                                // make sure bumping into walls doesnt take a turn
                            } else { continue }
                        }
                        data.command_event_channel.single_write(command_event);
                        data.world_resources.world_time.increment_player_turn();
                    }
                },
                _ => (),
            }
        }
    }
    
    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        let command_event_channel: EventChannel<CommandEvent> = EventChannel::new();
        world.insert(command_event_channel);
        let key_channel: EventChannel<VirtualKeyCode> = EventChannel::new();
        world.insert(key_channel);

        // incoming input events
        self.key_reader = Some(world.
            fetch_mut::<EventChannel<VirtualKeyCode>>()
            .register_reader());
    }
}