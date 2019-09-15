use specs::prelude::*;
use shrev::{EventChannel};
use tcod::map::FovAlgorithm;
use crate::command::{Command, CommandEvent};
use crate::components::{AiControl, MyTurn, Position, Target, Seeing};
use crate::systems::movement::{Dir};
use crate::map::{EntityMap, View};

// use std::sync::{Arc, Mutex};


#[derive(Debug, Copy, Clone)]
pub enum AiType {
    Dummy,
    MoveSE,
    _Friendly,
    _Monster,
}

pub struct Ai;
impl Ai {
    fn get_command(entity: Entity, ai_type: AiType, data: &<Ai as System>::SystemData) -> Option<Command> {
        match ai_type {
            AiType::Dummy => Some(Command::Move(Self::path_to_player(entity, data))),
            AiType::MoveSE => Some(Command::Move(Dir::SE)),
            _ => None,
        }
    }
    fn path_to_player(entity: Entity, data: &<Ai as System>::SystemData) -> Dir {
        if let (Some(target), Some(pos), Some(seer)) = (data.targets.get(entity), data.positions.get(entity), data.seers.get(entity)) {
            if let Some(dest) = data.positions.get(target.entity) {
                
                let mut fov_map = data.view.map.lock().unwrap();
                // fov_map.compute_fov(pos.x, pos.y, seer.fov, true, FovAlgorithm::Basic);

                if !fov_map.is_in_fov(dest.x, dest.y) {
                    return Dir::Nowhere
                }

                // copied this from tutorial lol
                let dx = dest.x - pos.x;
                let dy = dest.y - pos.y;
                let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

                // normalize it to length 1 (preserving direction), then round it and
                // convert to integer so the movement is restricted to the map grid
                let dx = (dx as f32 / distance).round() as i32;
                let dy = (dy as f32 / distance).round() as i32;
                return Dir::pos_to_dir((dx, dy))
            }
        }
        Dir::Nowhere
    }
    
}

#[derive(SystemData)]
pub struct AiSystemData<'a> {
    pub entities: Entities<'a>,
    pub entity_map: ReadExpect<'a, EntityMap>,
    pub positions:  ReadStorage<'a, Position>,
    pub targets:     ReadStorage<'a, Target>,
    pub ai_units:    ReadStorage<'a, AiControl>,
    pub seers: ReadStorage<'a, Seeing>,
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
        for (ent, ai_unit, _my_turn, _position) in (&data.entities, &data.ai_units, &data.my_turns, &data.positions).join() {
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
