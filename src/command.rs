use specs::prelude::*;
use crate::systems::movement::Dir;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Command {
    Move(Dir),
    Attack(Dir),
    _Use(Entity),
    _Rest,
    ToggleRealTime,
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