extern crate specs;

#[macro_use]
extern crate specs_derive;
extern crate shrev;
extern crate shred;
extern crate shred_derive;
extern crate toml;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate vecmap;

use specs::prelude::*;
use std::env;
use tcod::console::*;

mod config;
mod entities;
mod components;
mod map;
mod command;
mod time;
mod systems;
mod ecs;
mod bodyparts;
mod entity_factory;

use ecs::*;
use shrev::EventChannel;
use tcod::input::*;
use config::*;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    #[derive(Debug)]
    static ref CONFIG: Config = Config::open();
}

fn main() {
    let mut ecs= ecs::world_setup();
    ecs.main_loop();
}