extern crate specs;

#[macro_use]
extern crate specs_derive;
extern crate shred;

extern crate shred_derive;
extern crate shrev;

use specs::prelude::*;

use rand::prelude::*;
use rand::seq::SliceRandom;

use tcod::console::*;
use tcod::map::Map as TcodMap;

use std::sync::{Arc, Mutex};

pub const SCREEN_WIDTH: i32 = 85;
pub const SCREEN_HEIGHT: i32 = 45;

pub const MAP_HEIGHT: i32 = 512;
pub const MAP_WIDTH: i32 = 512;

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

fn main() {
    let mut DEBUG = false;
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("debug")) {
        DEBUG = true;
    }

    let mut world = World::new();
    let builder = DispatcherBuilder::new()
        .with(systems::initiative::Initiative, "initiative_sys", &[])
        .with_barrier()
        .with(systems::input::Input::new(), "input_sys", &[]) 
        .with(systems::ai::Ai, "ai_sys", &[])
        .with(systems::action::ActionHandler::new(), "action_sys", &["input_sys", "ai_sys"])
        .with(systems::movement::Movement::new(), "movement_sys", &["action_sys"])
        .with(systems::movement::CollisionMapUpdater::new(), "collision_map_updater_sys", &["movement_sys", "action_sys"])
        .with_barrier()
        .with_thread_local(systems::render::Render::new());

    let mut dispatcher = builder.build();
    dispatcher.setup(&mut world);
   
    let game_state = GameState { end: false, player_turn: false, real_time: false, debug: DEBUG };
    let root = Root::initializer()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .font("terminal.png", FontLayout::AsciiInCol)
        .init();
        
    let view = map::View { map: Arc::new(Mutex::new(TcodMap::new(MAP_WIDTH, MAP_HEIGHT))) };
    let map = map::EntityMap::new();
    world.insert(game_state);
    world.insert(root);
    world.insert(map);
    world.insert(view);

    let player = entities::create_player(&mut world, MAP_WIDTH/ 2, MAP_HEIGHT/2);
    entities::create_shack(&mut world, MAP_WIDTH/2, MAP_HEIGHT/2, 10);
    
    let mut dummies_list = vec![player]; 
    let mut rng = thread_rng();

    for _ in 0..1000 {
        dummies_list.push(entities::create_dummy(&mut world, dummies_list.choose(&mut rng).unwrap().clone()));
    }

    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            entities::create_floor(&mut world, x, y)
        }
    }

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