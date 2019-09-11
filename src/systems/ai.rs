use specs::prelude::*;
use shrev::{EventChannel};
use crate::command::{Command, CommandEvent};
use crate::components::{AiControl, MyTurn, Position, Target};
use crate::systems::movement::{Dir};
use crate::map::View;

// use std::sync::{Arc, Mutex};


#[derive(Debug, Copy, Clone)]
pub enum AiType {
    Dummy,
    _Friendly,
    _Monster,
}

pub struct Ai;
impl Ai {
    fn get_command(entity: Entity, ai_type: AiType, data: &<Ai as System>::SystemData) -> Option<Command> {
        match ai_type {
            AiType::Dummy => Some(Command::Move(Self::path_to(entity, data))),
            _ => None,
        }
    }
    fn path_to(_entity: Entity, _data: &<Ai as System>::SystemData) -> Dir {
        // pathfinding code here
        Dir::Nowhere
    }
}

#[derive(SystemData)]
pub struct AiSystemData<'a> {
    pub entities: Entities<'a>,
    pub positions:  ReadStorage<'a, Position>,
    pub targets:     ReadStorage<'a, Target>,
    pub ai_units:    ReadStorage<'a, AiControl>,
    pub command_event_channel:  Write<'a, EventChannel<CommandEvent>>,
    pub my_turns:   WriteStorage<'a, MyTurn>,
    pub world_updater:  Read<'a, LazyUpdate>,
    pub game_state: ReadExpect<'a, crate::GameState>,
    pub view: ReadExpect<'a, View>,
}

impl <'a> System<'a> for Ai {
    type SystemData = AiSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if data.game_state.player_turn {
            return;
        }
        for (ent, ai_unit, _my_turn) in (&data.entities, &data.ai_units, &data.my_turns).join() {
            let ai_type = ai_unit.ai_type;
            let command = Self::get_command(ent, ai_type, &data);
            match command {
                None => (),
                Some(_) => {
                    // attach action component to player entity 
                    let command_event = CommandEvent::new(command.unwrap(), ent);
                    data.command_event_channel.single_write(command_event);
                }
            }
            data.world_updater.remove::<MyTurn>(ent);
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        let command_event_channel: EventChannel<CommandEvent> = EventChannel::new();
        world.insert(command_event_channel);
    }
}
