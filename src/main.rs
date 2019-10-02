extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate shrev;
extern crate shred;
extern crate shred_derive;

#[macro_use]
extern crate log;
extern crate env_logger;

use specs::prelude::*;
use std::env;
use tcod::console::*;

mod entities;
mod components;
mod map;
mod command;
mod time;
mod systems;
mod ecs;
mod render;
mod input;
use ecs::*;
use shrev::EventChannel;
use tcod::input::*;

pub const SCREEN_WIDTH: i32 = 60;
pub const SCREEN_HEIGHT: i32 = 35;

pub const VIEWPORT_WIDTH: i32 = SCREEN_WIDTH - 31;
pub const VIEWPORT_HEIGHT: i32 = SCREEN_HEIGHT - 2;

pub const MAP_WIDTH: i32 = 100;
pub const MAP_HEIGHT: i32 = 100;

pub const BASE_TURN_TIME: u32 = 1000;
pub const MIN_TURN_TIME: u32 = 1;

pub struct Ecs {
    world: World,
    dispatcher: Dispatcher<'static, 'static>,
}

impl Ecs {
    fn main_loop(&mut self) {
        loop {
            self.world.maintain();
            {
                let mut root = self.world.write_resource::<Root>();
                root.flush();
            }
            self.dispatcher.dispatch(&mut self.world);
        }
    }
}

fn main() {
    let mut debug = false;
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("debug")) { debug = true; }

    let (world, dispatcher) = ecs::world_setup(debug);
    let mut ecs = Ecs { world, dispatcher };

    ecs.main_loop();
}