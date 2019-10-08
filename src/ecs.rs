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
use vecmap::*;
use crate::systems::render::Tile;


pub struct GameState {
    pub player_turn: bool,
    pub real_time: bool,
    pub debug: bool,
    pub game_end: bool,
    pub world_time: time::WorldTime,
}

pub struct MessageLog {
    pub messages: Vec<String>,
}

impl MessageLog {
    pub fn new() -> Self {
        MessageLog {
            messages: Vec::new(),
        }
    }

    pub fn log(&mut self, string: String) {
        self.messages.insert(0, string);
    }

    pub fn _pop(&mut self) -> Option<String> {
        match self.messages.len() {
            0 => None,
            _ => Some(self.messages.remove(0))
        }
    }
}

pub struct Ecs {
    world: World,
    dispatcher: Dispatcher<'static, 'static>,
}

impl Ecs {
    pub fn main_loop(&mut self) {
        loop {
            self.world.maintain();
            {
                let game_state = self.world.read_resource::<GameState>();
                let mut root = self.world.write_resource::<Root>();
                root.flush();
                if root.window_closed() || game_state.game_end { break }
            }
            self.dispatcher.dispatch(&mut self.world);
        }
    }
}

//pub struct

pub fn world_setup<'a, 'b> (debug: bool) -> Ecs {
    let mut world = World::new();
    let builder = DispatcherBuilder::new()
//         .with(systems::mapgen::MapGen::new(), "map_gen_sys", &[])
        .with(systems::naming::Naming, "naming_sys", &[])
        .with_barrier()
        .with(systems::input::InputListener, "input_listener_sys", &[])
        .with(systems::movement::CollisionMapUpdater::new(), "collision_map_updater_sys", &[])
        .with(systems::ai::Ai, "ai_sys", &[])
        .with(systems::time::TurnAllocator, "turn_allocator_sys", &[])
        .with(systems::time::PlayerStartTurn, "player_start_turn_sys", &["turn_allocator_sys"])
        .with(systems::stats::QuicknessSystem, "quickness_sys", &[])
//        .with_barrier()
        .with(systems::input::Input::new(), "input_sys", &[])
        .with(systems::action::ActionHandler::new(), "action_sys", &["ai_sys"])
        .with(systems::movement::Movement, "movement_sys", &["action_sys"])
        .with(systems::combat::Attack, "attack_sys", &["movement_sys", "action_sys"])
        .with(systems::combat::Defend, "defend_sys", &["attack_sys"])
//        .with_barrier()
        .with(systems::time::EndTurn, "end_turn_sys", &[])
        .with(systems::render::RenderViewport::new(), "render_viewport_sys", &[])
        .with(systems::render::RenderUi, "render_ui_sys", &[]);

    let mut dispatcher = builder.build();
    dispatcher.setup(&mut world);

    let world_time = time::WorldTime::new();
    let game_state = GameState {
        player_turn: false,
        real_time: false,
        game_end: false,
        debug,
        world_time,
    };

    let view = map::View { map: Arc::new(Mutex::new(TcodMap::new(MAP_WIDTH, MAP_HEIGHT))) };
    let map = map::EntityMap::new(MAP_WIDTH as usize, MAP_HEIGHT as usize);
    let message_log = MessageLog::new();
    let root = Root::initializer()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .font("term3.png", FontLayout::AsciiInRow)
        .init();

    world.insert(game_state);
    world.insert(map);
    world.insert(view);
    world.insert(message_log);
    world.insert(time::TurnQueue::new());
    world.insert(VecMap::<Tile>::filled_with(Tile::new(), MAP_WIDTH, MAP_HEIGHT));
    world.insert(RandomNumberGenerator::new());
    world.insert(root);

    entities::create_test_map(&mut world);
    dispatcher.dispatch(&mut world);

    Ecs { world, dispatcher }
}

