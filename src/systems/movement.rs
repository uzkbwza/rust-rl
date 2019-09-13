use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use crate::map::EntityMap;
use crate::components::{Position, Collidable, CostMultiplier};
use crate::map::View;

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
    pub cost: f32,
    }


impl MoveEvent {
    pub fn new(entity: Entity, start_x: i32, start_y: i32, dest_x: i32, dest_y: i32) -> Self {
        let cost: f32 = match i32::abs(dest_x-start_x) + i32::abs(dest_y-start_y) {
            2 => f32::sqrt(2.0),
            0 => 0.0,
            _ => 1.0,
        };

        MoveEvent {
            entity,
            start_x,
            start_y,
            dest_x,
            dest_y,
            cost,
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
        if position.x >= crate::MAP_WIDTH {
            position.x = 0;
        } else if position.x <= -1 {
            position.x = crate::SCREEN_WIDTH - 1;
        }
        if position.y >= crate::MAP_HEIGHT {
            position.y = 0;
        } else if position.y <= -1 {
            position.y = crate::MAP_HEIGHT - 1;
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
    pub cost_multipliers: WriteStorage<'a, CostMultiplier>,
    pub positions: WriteStorage<'a, Position>,
    pub collidables: ReadStorage<'a, Collidable>,
    pub entity_map: WriteExpect<'a, EntityMap>,
    pub view: WriteExpect<'a, View>,

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
        
        let mut view = data.view.map.lock().unwrap();
        
        for move_command in move_commands {
            let ent = move_command.entity;
            if let Some(pos) = data.positions.get_mut(ent) {
                let dest = (pos.x + move_command.dx, pos.y + move_command.dy);
                if data.entity_map.colliders.get(&dest) == None || data.collidables.get(ent) == None {
                    let move_event = Self::move_position(ent, pos, move_command);
                    data.move_event_channel.single_write(move_event);
                    if let Some(cost_multiplier) = &mut data.cost_multipliers.get_mut(ent) {
                        cost_multiplier.multiplier = move_event.cost
                    }

                    let (x, y) = (move_event.start_x, move_event.start_y);
                    let (dx, dy) = (move_event.dest_x, move_event.dest_y);

                    // remove collider from previous position
                    data.entity_map.colliders.remove(&(x, y));
                    view.set(x, y, true, true);

                    data.entity_map.colliders.insert((dx, dy), ent);
                    view.set(dx, dy, true, false);
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
    initialized: bool,
}

impl CollisionMapUpdater {
    pub fn new() -> Self {
        CollisionMapUpdater { 
            initialized: false
        }
    }
}

#[derive(SystemData)]
pub struct CollisionMapUpdaterSystemData<'a> {
    pub entities: Entities<'a>,
    pub collidables: ReadStorage<'a, Collidable>,
    pub positions: ReadStorage<'a, Position>,
    pub entity_map: WriteExpect<'a, EntityMap>,
    pub view: WriteExpect<'a, View>,
}

impl<'a> System<'a> for CollisionMapUpdater {
    type SystemData = CollisionMapUpdaterSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // for event in data.move_command_channel.read
        
        let mut view = data.view.map.lock().unwrap();
        // populate collision map

        if !self.initialized { 
            for (ent, pos) in (&data.entities, &data.positions).join() {
                if let Some(_collidable) = data.collidables.get(ent) {
                    data.entity_map.colliders.insert((pos.x, pos.y), ent);
                    view.set(pos.x, pos.y, true, false);
                } else {
                    view.set(pos.x, pos.y, true, true);
                }
            }
            self.initialized = true
        }
    }
}