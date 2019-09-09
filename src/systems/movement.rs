use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use crate::map::EntityMap;
use crate::components::{Position, Collidable};


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

pub struct Movement {
        pub move_command_reader: Option<ReaderId<MoveCommand>>
}

impl Movement {
    pub fn new() -> Self {
        Movement { 
            move_command_reader: None, 
        }
    }

    fn move_position(entity: Entity, position: &mut Position, move_command: &MoveCommand) -> MoveEvent {
        let start_x = position.x;
        let start_y = position.y;
        position.x += move_command.dx;
        position.y += move_command.dy;
        if position.x >= crate::SCREEN_WIDTH {
            position.x = 0;
        } else if position.x <= -1 {
            position.x = crate::SCREEN_WIDTH - 1;
        }
        if position.y >= crate::SCREEN_HEIGHT {
            position.y = 0;
        } else if position.y <= -1 {
            position.y = crate::SCREEN_HEIGHT - 1;
        }
        MoveEvent::new(
            entity,
            start_x,
            start_y,
            position.x,
            position.y,
        )
    }
}

#[derive(SystemData)]
pub struct MovementSystemData<'a> {
    pub entities: Entities<'a>,
    pub positions: WriteStorage<'a, Position>,
    pub collidables: ReadStorage<'a, Collidable>,
    pub entity_map: ReadExpect<'a, EntityMap>,

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
        
        for move_command in move_commands {
            let ent = move_command.entity;
            if let Some(pos) = data.positions.get_mut(ent) {
                let dest = (pos.x + move_command.dx, pos.y + move_command.dy);
                if data.entity_map.colliders.get(&dest) == None {
                    let move_event = Self::move_position(ent, pos, move_command);
                    data.move_event_channel.single_write(move_event);
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

pub struct CollisionMapUpdater {
    move_event_reader: Option<ReaderId<MoveEvent>>
}

impl CollisionMapUpdater {
    pub fn new() -> Self {
        CollisionMapUpdater { 
            move_event_reader: None, 
        }
    }
}

#[derive(SystemData)]
pub struct CollisionMapUpdaterSystemData<'a> {
    collidables: ReadStorage<'a, Collidable>,
    entity_map: WriteExpect<'a, EntityMap>,

    // read channels
    pub move_event_channel: Read<'a, EventChannel<MoveEvent>>,
}

impl<'a> System<'a> for CollisionMapUpdater {
    type SystemData = CollisionMapUpdaterSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // for event in data.move_command_channel.read
        let move_events = data.move_event_channel
            .read(self.move_event_reader
            .as_mut()
            .unwrap());
        
        for move_event in move_events {
            let ent = move_event.entity;
            if let Some(_collidable) = data.collidables.get(ent) {
                // remove collider from previous
                data.entity_map.colliders
                    .remove(&(move_event.start_x, move_event.start_y));

                data.entity_map.colliders
                    .insert((move_event.dest_x, move_event.dest_y), ent);
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.move_event_reader = Some(world.
            fetch_mut::<EventChannel<MoveEvent>>()
            .register_reader());
    }
}