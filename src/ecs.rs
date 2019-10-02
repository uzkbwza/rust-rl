use specs::prelude::*;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, MAP_WIDTH, MAP_HEIGHT};
use crate::time;
use crate::map;
use crate::systems;
use crate::entities;
use rltk::RandomNumberGenerator;
use std::sync::{Arc, Mutex};
use tcod::map::Map as TcodMap;
use tcod::console::*;


// previously GameState
pub struct WorldResources {
    pub player_turn: bool,
    pub real_time: bool,
    pub debug: bool,
    pub world_time: time::WorldTime,
}

pub struct MessageLog {
    pub messages: Vec<String>,
    pub capacity: usize,
}

impl MessageLog {
    pub fn new(capacity: usize) -> Self {
        MessageLog {
            messages: Vec::new(),
            capacity
        }
    }

    pub fn log(&mut self, string: String) {
        self.messages.insert(0, string);
        if self.messages.len() > self.capacity {
            self.messages.remove(self.capacity);
        }
    }

    pub fn _pop(&mut self) -> Option<String> {
        match self.messages.len() {
            0 => None,
            _ => Some(self.messages.remove(0))
        }
    }
}

//pub struct

pub fn world_setup<'a, 'b> (debug: bool) -> (World, Dispatcher<'a, 'b>) {
    let mut world = World::new();
    let builder = DispatcherBuilder::new()
//         .with(systems::mapgen::MapGen::new(), "map_gen_sys", &[])
        .with(systems::input::InputListener, "input_listener_sys", &[])
        .with(systems::movement::CollisionMapUpdater::new(), "collision_map_updater_sys", &[])
        .with(systems::ai::Ai, "ai_sys", &[])
        .with(systems::time::TurnAllocator, "turn_allocator_sys", &[])
        .with(systems::time::PlayerStartTurn, "player_start_turn_sys", &["turn_allocator_sys"])
//        .with_barrier()
        .with(systems::stats::QuicknessSystem, "quickness_sys", &[])
//        .with_barrier()
        .with(systems::input::Input::new(), "input_sys", &[])
        .with(systems::action::ActionHandler::new(), "action_sys", &["ai_sys"])
        .with(systems::movement::Movement, "movement_sys", &["action_sys"])
        .with(systems::attack::Attack, "attack_sys", &["movement_sys", "action_sys"])
//        .with_barrier()
        .with(systems::time::EndTurn, "end_turn_sys", &[])
        .with(systems::render::RenderViewport::new(), "render_viewport_sys", &[]);

    let mut dispatcher = builder.build();
    dispatcher.setup(&mut world);

    let world_time = time::WorldTime::new();
    let world_resources = WorldResources {
        player_turn: false,
        real_time: false,
        debug,
        world_time,
    };

    let view = map::View { map: Arc::new(Mutex::new(TcodMap::new(MAP_WIDTH, MAP_HEIGHT))) };
    let map = map::EntityMap::new(MAP_WIDTH as usize, MAP_HEIGHT as usize);
    let message_log = MessageLog::new(30);
    let root = Root::initializer()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .font("Anno_16x16.png", FontLayout::AsciiInRow)
        .init();

    world.insert(world_resources);
    world.insert(map);
    world.insert(view);
    world.insert(message_log);
    world.insert(time::TurnQueue::new());
    world.insert(systems::render::TileMap::filled_with(None));
    world.insert(RandomNumberGenerator::new());
    world.insert(root);

    entities::create_test_map(&mut world);
    dispatcher.dispatch(&mut world);

    (world, dispatcher)
}

