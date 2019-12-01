extern crate specs;

#[macro_use]
extern crate specs_derive;
extern crate shred;
extern crate shred_derive;
extern crate shrev;
extern crate toml;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate mapgen;
extern crate vecmap;

#[macro_use]
extern crate arrayref;
extern crate sha2;

use specs::prelude::*;
use std::env;
use tcod::console::*;

mod bodyparts;
mod command;
mod components;
mod config;
mod ecs;
mod entity_factory;
mod map;
mod systems;
mod time;

use config::*;
use ecs::*;
use shrev::EventChannel;
use tcod::input::*;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    #[derive(Debug)]
    static ref CONFIG: Config = Config::open();
}

fn main() {
    let mut ecs = ecs::world_setup();

    ecs.main_loop();
}
