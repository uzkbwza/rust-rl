extern crate specs;

#[macro_use]
extern crate specs_derive;
extern crate shred;

extern crate shred_derive;
extern crate shrev;

use specs::prelude::*;
use tcod::console::*;

mod entities;
mod components;
mod systems;
mod prelude;

use prelude::*;

const SCREEN_WIDTH: i32 = 90;
const SCREEN_HEIGHT: i32 = 60;

const DEBUG: bool = true;

fn main() {
    let mut world = World::new();
    let mut builder = DispatcherBuilder::new()
        .with(systems::control::Input, "input_sys", &[])
        .with(systems::action::ActionHandler::new(), "action_sys", &["input_sys"])
        .with(systems::movement::Movement::new(), "movement_sys", &["action_sys"])
        .with(systems::render::Render, "render_sys", &[]);

    if DEBUG {
        builder.add(systems::debug::DEBUG::new(), "debug_sys", &[])
    }

    let mut dispatcher = builder.build();

    dispatcher.setup(&mut world);
    
    let root = Root::initializer()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .font("terminal.png", FontLayout::AsciiInCol)
        .init();
    
    tcod::system::set_fps(60);

    let game_state = GameState { end: false };
    world.insert(game_state);
    world.insert(root);
    entities::create_player(&mut world, SCREEN_WIDTH/2, SCREEN_HEIGHT/2);

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