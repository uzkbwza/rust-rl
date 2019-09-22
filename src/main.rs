extern crate specs;

#[macro_use]
extern crate specs_derive;
extern crate shred;

extern crate shred_derive;
extern crate shrev;

use specs::prelude::*;

use rand::prelude::*;

use tcod::console::*;
use tcod::map::Map as TcodMap;

use std::sync::{Arc, Mutex};

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;

pub const MAP_WIDTH: i32 = 48;
pub const MAP_HEIGHT: i32 = 48;

use std::env;

mod entities;
mod components;
mod systems;
mod map;
mod command;


// use prelude::*;
pub struct GameState {
    pub end: bool,
    pub player_turn: bool,
    pub real_time: bool,
    pub debug: bool,
}

pub struct MessageLog {
    messages: Vec<String>,
    capacity: usize,
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

    pub fn pop(&mut self) -> Option<String> {
        match self.messages.len() {
            0 => None,
            _ => Some(self.messages.remove(0))
        }
    }
}

fn main() {
    let mut DEBUG = false;
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("debug")) {
        DEBUG = true;
    }

    let mut world = World::new();
    let builder = DispatcherBuilder::new()
        .with(systems::mapgen::MapGen::new(), "map_gen_sys", &[])
        .with(systems::initiative::Initiative, "initiative_sys", &[])
        .with(systems::movement::CollisionMapUpdater::new(), "collision_map_updater_sys", &[])
        .with(systems::input::Input::new(), "input_sys", &[]) 
        .with(systems::ai::Ai, "ai_sys", &[])
        .with(systems::action::ActionHandler::new(), "action_sys", &["input_sys", "ai_sys"])
        .with(systems::movement::Movement, "movement_sys", &["action_sys"])
        .with(systems::attack::Attack, "attack_sys", &["movement_sys", "action_sys"])
        .with_thread_local(systems::render::Render::new());

    let mut dispatcher = builder.build();
    dispatcher.setup(&mut world);
   
    let game_state = GameState { end: false, player_turn: false, real_time: false, debug: DEBUG };
    let root = Root::initializer()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .font("terminal2.png", FontLayout::AsciiInCol)
        .init();
        
    let view = map::View { map: Arc::new(Mutex::new(TcodMap::new(MAP_WIDTH, MAP_HEIGHT))) };
    let map = map::EntityMap::new(MAP_WIDTH as usize, MAP_HEIGHT as usize);
    let message_log = MessageLog::new(30);
    world.insert(game_state);
    world.insert(root);
    world.insert(map);
    world.insert(view);
    world.insert(message_log);

    let player = entities::create_player(&mut world, MAP_WIDTH/2, MAP_HEIGHT/2);
    
    let mut dummies_list = vec![player]; 
    let mut rng = thread_rng();

    // for x in 0..MAP_WIDTH {
    //     for y in 0..MAP_HEIGHT {
    //         entities::create_floor(&mut world, x, y);
    //     }
    // }

    // for y in 0..MAP_HEIGHT {
    //     entities::create_wall(&mut world, MAP_WIDTH - 1, y);
    //     entities::create_wall(&mut world, 0, y);
    // }
    // for x in 0..MAP_WIDTH {
    //     entities::create_wall(&mut world, x, 0);
    //     entities::create_wall(&mut world, x, MAP_HEIGHT - 1);
    // }

    // entities::create_shack(&mut world, MAP_WIDTH/2, MAP_HEIGHT/2, 7);

    // for _ in 0..5 {
    //     dummies_list.push(entities::create_dummy(&mut world, player));
    // }

    loop {
        dispatcher.dispatch(&mut world);
        {
            let game_state = world.read_resource::<GameState>();
            let root = world.read_resource::<Root>();
            if game_state.end || root.window_closed() { break }
        }
        world.maintain();
    }
}