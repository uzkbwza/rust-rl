extern crate specs;

#[macro_use]
extern crate specs_derive;
extern crate shred;

extern crate shred_derive;
extern crate shrev;

use specs::prelude::*;
// use specs::world;
use tcod::console::*;

pub const SCREEN_WIDTH: i32 = 120;
pub const SCREEN_HEIGHT: i32 = 70;
pub const DEBUG: bool = true;

mod entities;
mod components;
mod systems;
mod prelude;
mod map;

// use prelude::*;

pub struct GameState {
    pub end: bool,
    pub player_turn: bool,
    pub real_time: bool,
    pub debug: bool,
}

fn main() {
    let mut world = World::new();
    let mut builder = DispatcherBuilder::new()
        .with(systems::initiative::Initiative, "initiative_sys", &[])
        .with_barrier()
        .with(systems::control::Ai, "ai_sys", &[])
        .with_barrier()
        .with(systems::control::Input::new(), "input_sys", &[])
        .with(systems::action::ActionHandler::new(), "action_sys", &["input_sys", "ai_sys"])
        .with(systems::movement::Movement::new(), "movement_sys", &["action_sys"])
        .with(systems::movement::CollisionMapUpdater::new(), "collision_map_updater_sys", &["action_sys"])
        .with_barrier()
        .with_thread_local(systems::render::Render::new());

    if DEBUG {
        builder.add(systems::debug::DEBUG::new(), "debug_sys", &[])
    }
    let mut dispatcher = builder.build();
    dispatcher.setup(&mut world);
   
    let game_state = GameState { end: false, player_turn: false, real_time: false, debug: DEBUG };
    let root = Root::initializer()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .font("terminal2.png", FontLayout::AsciiInCol)
        .init();
    
    world.insert(game_state);
    world.insert(root);
    
    entities::create_player(&mut world, SCREEN_WIDTH/2, SCREEN_HEIGHT/2);
    for _ in 0..1000 {
        entities::create_dummy(&mut world);
    }
    // tcod::system::set_fps(60);

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