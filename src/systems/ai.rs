use specs::prelude::*;
use shrev::{EventChannel};
use tcod::map::FovAlgorithm;
use tcod::map::Map as TcodMap;
use crate::command::{Command, CommandEvent};
use crate::components::{AiControl, MyTurn, Position, Target, Seeing, PlayerControl};
use crate::systems::movement::{Dir};
use crate::map::{EntityMap, View};
use tcod::pathfinding::Dijkstra;
use array2d::Array2D;


// use std::sync::{Arc, Mutex};


#[derive(Debug, Copy, Clone)]
pub enum AiType {
    Dummy,
    _Friendly,
    _Monster,
}

pub struct Ai;
impl Ai {
    fn get_command(entity: Entity, ai_type: AiType, data: &AiSystemData) -> Option<Command> {
        match ai_type {
            AiType::Dummy => Some(Command::Move(Self::path_to_target(entity, data))),
            _ => None,
        }
    }

    fn rank_distance(start: (i32, i32), dest: (i32, i32), point: (i32, i32), fov_map: &TcodMap, mut pathfinder: &mut Dijkstra, entity_map: &EntityMap) -> Option<(i32, (i32, i32))> {
        let x = point.0;
        let y = point.1;
        pathfinder.find((x, y));
        let num_steps = pathfinder.len();
        // println!("{}", num_steps);
        let mut ranking = (((dest.0 - x).pow(2) + (dest.1 - y).pow(2)) as f32).sqrt() as i32 ;
        if !fov_map.is_walkable(x as i32, y as i32) {
            return None
        }

        // if entity_map.actors.contains_actor(x as i32, y as i32) {
        //     return None
        // }
        Some((ranking, (x, y)))
    }

    fn choose_close_point(range: i32, start: (i32, i32), dest: (i32, i32), fov_map: &TcodMap, mut pathfinder: &mut Dijkstra, entity_map: &EntityMap) -> (i32, i32) {
        let mut rankings = Vec::new();
        for x in (start.0 - range)..(start.0 + range) + 1  {
            for y in (start.1 - range)..(start.1 + range) + 1 {
                if x <= 0 ||
                 y <= 0 || x >= entity_map.width as i32 || y >= entity_map.height as i32 {
                    continue
                }
                if let Some(ranking) = Self::rank_distance(start, dest, (x, y), fov_map, pathfinder, entity_map) {
                    rankings.push(ranking);
                }
            }
        }
        if rankings.is_empty() { return start }
        rankings.sort_by(|a, b| a.0.cmp(&b.0));
        
        // println!("{:?}", rankings[0]);
        let closest = ((rankings[0].1).0 as i32, (rankings[0].1).1 as i32);
        closest
    }


    fn path_to_target(entity: Entity, data: &AiSystemData) -> Dir {
        if let (Some(target), Some(pos), Some(seer)) = (data.targets.get(entity), data.positions.get(entity), data.seers.get(entity)) {
            if let Some(dest) = data.positions.get(target.entity) {
                
                let mut fov_map = data.view.map.lock().unwrap();

                fov_map.compute_fov(pos.x, pos.y, seer.fov, true, FovAlgorithm::Basic);
                fov_map.set(pos.x, pos.y, true, true);

                let mut step_pos = (pos.x, pos.y);

                let mut pathfinder = Dijkstra::new_from_map(fov_map.clone(), f32::sqrt(2.0));
                pathfinder.compute_grid(step_pos);
                let dest_point = Self::choose_close_point(20, (pos.x, pos.y), (dest.x, dest.y), &fov_map, &mut pathfinder, &data.entity_map);

                if pathfinder.find((dest_point.0, dest_point.1)) {
                    if let Some(step) = pathfinder.get(0) {
                        step_pos = pathfinder.walk_one_step().unwrap(); 
                    }
                } else {
                    return Dir::Nowhere;
                }


                if pos.x == step_pos.0 && pos.y == step_pos.1 {
                    return Dir::Nowhere;
                }

                let dx = step_pos.0 - pos.x;
                let dy = step_pos.1 - pos.y;
                let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();
                let dx = (dx as f32 / distance).round() as i32;
                let dy = (dy as f32 / distance).round() as i32;

                return Dir::pos_to_dir((dx, dy))
            }
        }
        Dir::Nowhere
    }
    
    // fn target_player(entity: Entity, data: &AiSystemData) {
    //     if data.targets.get(entity) != None {
    //         return 
    //     }

    //     if let (Some(pos), Some(seer)) = (data.positions.get(entity), data.seers.get(entity)) {
    //         let mut fov_map = data.view.map.lock().unwrap();

    //         for (player_entity, target_pos, player) in (data.entities, data.positions, data.players).join() {
    //             fov_map.compute_fov(pos.x, pos.y, seer.fov, true, FovAlgorithm::Basic);
    //             if fov_map.is_in_fov(target_pos.x, target_pos.y) {
    //                 data.targets.insert(entity, Target { entity: player_entity });
    //             }
    //         } 
    //     }
    // }
}

#[derive(SystemData)]
pub struct AiSystemData<'a> {
    pub entities: Entities<'a>,
    pub players: ReadStorage<'a, PlayerControl>,
    pub entity_map: ReadExpect<'a, EntityMap>,
    pub positions:  ReadStorage<'a, Position>,
    pub targets:     WriteStorage<'a, Target>,
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
