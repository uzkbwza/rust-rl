use specs::prelude::*;
use shrev::{EventChannel};
use tcod::console::*;
use tcod::input::{KeyCode, KeyPressFlags};
use tcod::input::Key as TcodKey;
use crate::components::*;
use crate::prelude::*;
use crate::systems::movement::Dir;


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum KeyEvent {
    Escape, Enter, Backspace, Space,
    Ctrl, Shift, Alt,
    Up, Down, Left, Right,
    N1, N2, N3, N4, N5, N6, N7, N8, N9, N0,
    A, B, C, D, E, F, G, H, I, J, K, L, M, 
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z, _UNIMPLEMENTED
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Command {
    Move(Dir),
    _Use(Entity),
    EndGame,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CommandEvent {
    pub command: Command,
    pub entity: Entity,
}

impl CommandEvent {
    pub fn new(command: Command, entity: Entity) -> Self {
        CommandEvent { command, entity }
    }
}

pub struct Input;
impl Input {
    // tcod keys suck. this will make things a little easier
    fn get_key(key_state: Option<TcodKey>) -> Option<KeyEvent> {
        if key_state == None { return None }
        let key = key_state.unwrap();
        if !key.pressed { return None }
        match key.code {
            KeyCode::Escape     => Some(KeyEvent::Escape),
            KeyCode::Enter      => Some(KeyEvent::Enter),
            KeyCode::Backspace  => Some(KeyEvent::Backspace),
            KeyCode::Spacebar   => Some(KeyEvent::Space),
            KeyCode::Control    => Some(KeyEvent::Ctrl),
            KeyCode::Shift      => Some(KeyEvent::Shift),
            KeyCode::Alt        => Some(KeyEvent::Alt),
            KeyCode::Up         => Some(KeyEvent::Up),
            KeyCode::Down       => Some(KeyEvent::Down),
            KeyCode::Left       => Some(KeyEvent::Left),
            KeyCode::Right      => Some(KeyEvent::Right),
            KeyCode::Char => {
                match key.printable {
                    'a' => Some(KeyEvent::A),
                    'b' => Some(KeyEvent::B),
                    'c' => Some(KeyEvent::C),
                    'd' => Some(KeyEvent::D),
                    'e' => Some(KeyEvent::E),
                    'f' => Some(KeyEvent::F),
                    'g' => Some(KeyEvent::G),
                    'h' => Some(KeyEvent::H),
                    'i' => Some(KeyEvent::I),
                    'j' => Some(KeyEvent::J),
                    'k' => Some(KeyEvent::K),
                    'l' => Some(KeyEvent::L),
                    'm' => Some(KeyEvent::M),
                    'n' => Some(KeyEvent::N),
                    'o' => Some(KeyEvent::O),
                    'p' => Some(KeyEvent::P),
                    'q' => Some(KeyEvent::Q),
                    'r' => Some(KeyEvent::R),
                    's' => Some(KeyEvent::S),
                    't' => Some(KeyEvent::T),
                    'u' => Some(KeyEvent::U),
                    'v' => Some(KeyEvent::V),
                    'w' => Some(KeyEvent::W),
                    'x' => Some(KeyEvent::X),
                    'y' => Some(KeyEvent::Y),
                    'z' => Some(KeyEvent::Z),
                    _ => None
                }
            },
            KeyCode::Number1 => Some(KeyEvent::N1),
            KeyCode::Number2 => Some(KeyEvent::N2),
            KeyCode::Number3 => Some(KeyEvent::N3),
            KeyCode::Number4 => Some(KeyEvent::N4),
            KeyCode::Number5 => Some(KeyEvent::N5),
            KeyCode::Number6 => Some(KeyEvent::N6),
            KeyCode::Number7 => Some(KeyEvent::N7),
            KeyCode::Number8 => Some(KeyEvent::N8),
            KeyCode::Number9 => Some(KeyEvent::N9),
            KeyCode::Number0 => Some(KeyEvent::N0),
            _ => None
        }
    }

    fn get_command_from_key(key: KeyEvent) -> Option<Command> {
        match key {
            // global commands
            KeyEvent::Escape => Some(Command::EndGame),

            // actor commands
            KeyEvent::H => Some(Command::Move(Dir::W)),
            KeyEvent::J => Some(Command::Move(Dir::S)),
            KeyEvent::K => Some(Command::Move(Dir::N)),
            KeyEvent::L => Some(Command::Move(Dir::E)),
            KeyEvent::Y => Some(Command::Move(Dir::NW)),
            KeyEvent::U => Some(Command::Move(Dir::NE)),
            KeyEvent::B => Some(Command::Move(Dir::SW)),
            KeyEvent::N => Some(Command::Move(Dir::SE)),
            _ => None
        }
    }
}


#[derive(SystemData)]
pub struct InputSystemData<'a> {
    pub entities:   Entities<'a>,
    pub players:    ReadStorage<'a, PlayerControl>,
    pub root:       ReadExpect<'a, Root>,
    pub game_state: WriteExpect<'a, GameState>,
    pub key_event_channel:     Write<'a, EventChannel<KeyEvent>>,
    pub command_event_channel:       Write<'a, EventChannel<CommandEvent>>,
}

impl<'a> System<'a> for Input {
    type SystemData = InputSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {

        let key_state = data.root.check_for_keypress(KeyPressFlags::all());
        let key = Self::get_key(key_state);

        if key == None { 
            return 
        } 
        let key = key.unwrap(); 

        data.key_event_channel.single_write(key);
        
        let command = Self::get_command_from_key(key);

        for (ent, _player) in (&data.entities, &data.players).join() {
            match command {
                None => (),

                // global commands
                Some(Command::EndGame) => data.game_state.end = true,
                
                // player commands
                Some(_) => {

                    // attach action component to player entity 
                    let command_event = CommandEvent::new(command.unwrap(), ent);
                    data.command_event_channel.single_write(command_event);
                },
            }
        }
    }
    
    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        let key_event_channel: EventChannel<KeyEvent> = EventChannel::new();
        let command_event_channel: EventChannel<CommandEvent> = EventChannel::new();
        world.insert(key_event_channel);
        world.insert(command_event_channel);
    }
}