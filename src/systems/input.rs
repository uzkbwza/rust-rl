use specs::prelude::*;
use shrev::{EventChannel};

use crate::command::{Command, CommandEvent};
use crate::map::*;
use crate::components::{PlayerControl, MyTurn, Position};
use crate::systems::movement::{Dir};
use tcod::input::*;
use tcod::console::*;
use crate::CONFIG;

#[derive(Debug)]
pub struct Input {
    command_queue: Vec<Command>,
    key_reader: Option<ReaderId<Key>>,
}

pub trait KeyInterface {

}

impl Input {
    pub fn new() -> Self {
        Input {
            command_queue: Vec::new(),
            key_reader: None,
        }
    }

    fn get_command_from_key(key: Key) -> Option<Command> {
        match key.code {
            KeyCode::Escape => Some(Command::EndGame),

            KeyCode::Char => match key.printable {
                // actor commands
                'h' => Some(Command::Move(Dir::W)),
                'j' => Some(Command::Move(Dir::S)),
                'k' => Some(Command::Move(Dir::N)),
                'l' => Some(Command::Move(Dir::E)),
                'y' => Some(Command::Move(Dir::NW)),
                'u' => Some(Command::Move(Dir::NE)),
                'b' => Some(Command::Move(Dir::SW)),
                'n' => Some(Command::Move(Dir::SE)),
                '.' => Some(Command::Move(Dir::Nowhere)),
                _ => None,
            },

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
    pub game_state: WriteExpect<'a, crate::GameState>,
    pub key_channel:      Read<'a, EventChannel<Key>>,
    pub command_event_channel:  Write<'a, EventChannel<CommandEvent>>,
}

impl<'a> System<'a> for Input {
    type SystemData = InputSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {

        match self.key_reader { None => return, _ => () }
        let keys = data.key_channel.read(self.key_reader.as_mut().unwrap());
        for key in keys {
            if self.command_queue.len() < 3 {
                if let Some(command) = Self::get_command_from_key(*key) {
                    self.command_queue.push(command);
                }
            }
        }

        if self.command_queue.is_empty() { return }
//        println!("{:?}", self.command_queue);
        let command = self.command_queue.pop();

        for (ent, _player, _my_turn) in (&data.entities, &data.players, &mut data.my_turns).join() {
            match command {
                None => return,
                // meta commands
                Some(Command::EndGame) => { data.game_state.game_end = true }

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

//                        println!("{}, {}", dest.0, dest.1);

                        if (dest.0 as usize) >= CONFIG.map_width as usize
                            || (dest.1 as usize) >= CONFIG.map_height as usize
                            || (dest.0 as usize) < 0
                            || (dest.1 as usize) < 0 {

                            continue
                        }

                        if !fov_map.is_walkable(dest.0, dest.1) && dir != Dir::Nowhere {

                            // attack enemy instead if closeby
                            if let Ok(_) = data.entity_map.actors.retrieve(dest.0, dest.1) {
                                command_event = CommandEvent::new(Command::Attack(dir), ent);

                                // make sure bumping into walls doesnt take a turn
                            } else { continue }
                        }
                        data.command_event_channel.single_write(command_event);
                        // println!("{:?}", command_event);
                        data.game_state.world_time.increment_player_turn();
                    }
                },
                _ => (),
            }
//            println!("{:?}", command);
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        let command_event_channel: EventChannel<CommandEvent> = EventChannel::new();
        world.insert(command_event_channel);

        // incoming input events
        self.key_reader = Some(world.
            fetch_mut::<EventChannel<Key>>()
            .register_reader());
    }
}

pub struct InputListener;

#[derive(SystemData)]
pub struct InputListenerSystemData<'a> {
    pub key_channel:      Write<'a, EventChannel<Key>>,
    pub console: ReadExpect<'a, Root>,
}

impl<'a> System<'a> for InputListener {
    type SystemData = InputListenerSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let key = data.console.check_for_keypress(KeyPressFlags::all());
        if let Some(key_pressed) = key {
            if key_pressed.pressed {
                data.key_channel.single_write(key_pressed);
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        let key_channel: EventChannel<Key> = EventChannel::new();
        world.insert(key_channel);
    }
}