use specs::prelude::*;
use crate::map::*;
use crate::components::*;
use crate::components::flags::requests::*;
use crate::map::View;
use tcod::map::Map as TcodMap;
use vecmap::*;
use crate::CONFIG;
use crate::components::flags::ActionResult;

// use crate::systems::control::{CommandEvent};

// remember that commands are *requesting* an action, and events
// are the result of an action having happened.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Dir {
    N,
    S,
    E,
    W,
    NW,
    SW,
    NE,
    SE,
    Nowhere,
}

impl Dir {
        pub fn dir_to_pos(dir: Dir) -> (i32, i32) {
            match dir {
                Dir::N => (0, -1),
                Dir::S => (0, 1),
                Dir::E => (1, 0),
                Dir::W => (-1, 0),
                Dir::NW => (-1, -1),
                Dir::NE => (1, -1),
                Dir::SW => (-1, 1),
                Dir::SE => (1, 1),
                Dir::Nowhere => (0, 0),
            }
    }

    pub fn pos_to_dir(pos: (i32, i32)) -> Dir {
            match pos {
                (0, -1) => Dir::N,
                (0, 1) => Dir::S,
                (1, 0) => Dir::E,
                (-1, 0) => Dir::W, 
                (-1, -1) => Dir::NW,
                (1, -1) => Dir::NE,
                (-1, 1) => Dir::SW,
                (1, 1) =>  Dir::SE,
                _ =>  Dir::Nowhere,
            }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct MoveEvent {
    pub entity: Entity,
    pub start_x: i32,
    pub start_y: i32,
    pub dest_x: i32,
    pub dest_y: i32,
}


impl MoveEvent {
    pub fn new(entity: Entity, start_x: i32, start_y: i32, dest_x: i32, dest_y: i32) -> Self {
        MoveEvent {
            entity,
            start_x,
            start_y,
            dest_x,
            dest_y,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CollisionEvent {
    collider: Entity,
    collidee: Entity,
}


impl CollisionEvent {
    pub fn _new(collider: Entity, collidee: Entity) -> Self {
        CollisionEvent {
            collider,
            collidee,
        }
    }
}

pub struct Movement;

impl Movement {
    fn try_move_position(entity: Entity, position: &mut Position, move_command: &MoveRequest, view: &TcodMap, actor_map: &ActorMap) -> MoveEvent {
        let start_x = position.x;
        let start_y = position.y;
        let mut dest_x = position.x +  move_command.dx;
        let mut dest_y = position.y + move_command.dy;

        // if out of bounds, dont go anywhere
        if dest_x >= CONFIG.map_width || dest_x < 0 || dest_y >= CONFIG.map_height || dest_y < 0 {
            return MoveEvent::new(
                entity,
                start_x,
                start_y,
                start_x,
                start_y,
            )
        }

        if !view.is_walkable(dest_x, dest_y) || actor_map.retrieve(dest_x, dest_y).unwrap() != None {
            dest_x = start_x;
            dest_y = start_y;
        }

        if dest_x >= CONFIG.map_width {
            dest_x = 0;
        } else if dest_x <= -1 {
            dest_x = CONFIG.screen_width - 1;
        }
        if dest_y >= CONFIG.map_height {
            dest_y = 0;
        } else if dest_y <= -1 {
            dest_y = CONFIG.map_height - 1;
        }

        position.x = dest_x;
        position.y = dest_y;

        MoveEvent::new(
            entity,
            start_x,
            start_y,
            position.x,
            position.y,
        )
    }

    fn get_cost(base: u32, modifier: f32) -> u32 {
        (modifier * base as f32) as u32
    }
}


#[derive(SystemData)]
pub struct MovementSystemData<'a> {
    pub entities: Entities<'a>,
    pub positions: WriteStorage<'a, Position>,
    pub entity_map: WriteExpect<'a, EntityMap>,
    pub floors: ReadStorage<'a, Floor>,
    pub view: WriteExpect<'a, View>,
    pub world_updater: Read<'a, LazyUpdate>,
    pub action_results: WriteStorage<'a, ActionResult>,
    pub mobiles: ReadStorage<'a, Mobile>,

    // requests
    pub move_requests: WriteStorage<'a, MoveRequest>,
    pub attack_requests: WriteStorage<'a, AttackRequest>,
}

impl<'a> System<'a> for Movement {
    type SystemData = MovementSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        
        let mut view = data.view.map.lock().unwrap();
        
        for (ent, move_request) in (&data.entities, &mut data.move_requests).join() {
            data.world_updater.remove::<MoveRequest>(ent);
            // println!("removed moverequest");

            if let Some(pos) = data.positions.get_mut(ent) {
                // println!("got here");

                let actor_map = &data.entity_map.actors;
                let move_event = Self::try_move_position(ent, pos, move_request, &view, actor_map);

                // diagonals cost should be more
                let cost_modifier: f32 = match i32::abs(move_event.dest_x-move_event.start_x) + i32::abs(move_event.dest_y-move_event.start_y) {
                    2 => f32::sqrt(2.0),
                    1 => 1.0,
                    _ => 1.0,
                };

                let cost = match data.mobiles.get(ent) {
                    Some(mobile) => Self::get_cost(mobile.quickness, cost_modifier),
                    None => Self::get_cost(CONFIG.base_turn_time, cost_modifier),
                };

                let (x, y) = (move_event.start_x, move_event.start_y);
                let (dx, dy) = (move_event.dest_x, move_event.dest_y);

                // remove collider from previous position
                match data.entity_map.actors.reset_point(x, y) {
                    Ok(_) => (),
                    Err(e) => println!("{}", e)
                }
                view.set(x, y, true, true);

                match data.entity_map.actors.set_point(dx, dy, Some(ent)) {
                    Ok(_) => (),
                    Err(e) => println!("{}", e)
                }
                view.set(dx, dy, true, false);

                if let Err(err) = data.action_results.insert(ent, ActionResult::from(cost)) {
                    error!("Failed to insert action result from Movement system: {}", err)
                }
            }
        }
    }
}

pub struct CollisionMapUpdater;

impl CollisionMapUpdater {
    pub fn new() -> Self {
        CollisionMapUpdater {}
    }
}

#[derive(SystemData)]
pub struct CollisionMapUpdaterSystemData<'a> {
    pub entities: Entities<'a>,
    pub actors: ReadStorage<'a, Actor>,
    pub sight_blockers: ReadStorage<'a, BlockSight>,
    pub movement_blockers: ReadStorage<'a, BlockMovement>,
    pub positions: ReadStorage<'a, Position>,
    pub entity_map: WriteExpect<'a, EntityMap>,
    pub view: WriteExpect<'a, View>,
}

impl<'a> System<'a> for CollisionMapUpdater {
    type SystemData = CollisionMapUpdaterSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // for event in data.move_command_channel.read

        let mut view = data.view;
        let mut map = data.entity_map;

        map.actors.reset_map();
        view.block_map.reset_map();

        for (ent, pos) in (&data.entities, &data.positions).join() {
            let id = view.block_map.xy_idx(pos.x, pos.y);

            if let Some(_sight_blocker) = data.sight_blockers.get(ent) {
                view.block_map.items[id].blocks_sight = true
            }

            if let Some(_movement_blocker) = data.movement_blockers.get(ent) {
                view.block_map.items[id].blocks_movement = true
            }

            if let Some(_actor) = data.actors.get(ent) {
                match map.actors.set_point(pos.x, pos.y, Some(ent)) {
                    Ok(_) => (),
                    Err(e) => println!("{}", e),
                }
                view.block_map.items[id].blocks_movement = true
            }
        }

        for i in 0..view.block_map.items.len() {
            let (x, y) = view.block_map.idx_xy(i);
            let transparent = !view.block_map.items[i].blocks_sight;
            let walkable = !view.block_map.items[i].blocks_movement;
            view.map.lock().unwrap().set(x, y, transparent, walkable);
        }
    }
}