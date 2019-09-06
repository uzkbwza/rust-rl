use specs::prelude::*;
use shrev::{EventChannel, ReaderId};

use crate::components::*;
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
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct MoveCommand {
    entity: Entity,
    dx: i32,
    dy: i32,
}

impl MoveCommand {
    pub fn new(entity: Entity, dx: i32, dy: i32) -> Self {
        MoveCommand {
            entity,
            dx,
            dy,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct MoveEvent {
    entity: Entity,
    start_x: i32,
    start_y: i32,
    dest_x: i32,
    dest_y: i32,
}


impl MoveEvent {
    pub fn _new(entity: Entity, start_x: i32, start_y: i32, dest_x: i32, dest_y: i32) -> Self {
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
        }
}

pub struct Movement {
        pub move_command_reader: Option<ReaderId<MoveCommand>>
}

impl Movement {
    pub fn new() -> Self {
        Movement { 
            move_command_reader: None, 
        }
    }
}

#[derive(SystemData)]
pub struct MovementSystemData<'a> {
    pub entities: Entities<'a>,
    pub positions: WriteStorage<'a, Position>,

    // read channels
    pub move_command_channel: Read<'a, EventChannel<MoveCommand>>,

    // write channels
    pub move_event_channel: Write<'a, EventChannel<MoveEvent>>,
    pub collide_event_channel: Write<'a, EventChannel<CollisionEvent>>,
    
}

impl<'a> System<'a> for Movement {
    type SystemData = MovementSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let move_commands = data.move_command_channel
            .read(self.move_command_reader
            .as_mut()
            .unwrap());
        for command in move_commands {
            for (pos, entity) in (&mut data.positions, &data.entities).join() {
                if command.entity == entity {
                    pos.x += command.dx;
                    pos.y += command.dy;
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        let move_event_channel: EventChannel<MoveEvent> = EventChannel::new();
        let collision_event_channel: EventChannel<CollisionEvent> = EventChannel::new();
        world.insert(move_event_channel);
        world.insert(collision_event_channel);

        self.move_command_reader = Some(world.
            fetch_mut::<EventChannel<MoveCommand>>()
            .register_reader());
    }
}