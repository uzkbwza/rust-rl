extern crate specs;

#[macro_use]
extern crate specs_derive;
extern crate shrev;
extern crate shred;
extern crate shred_derive;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate vecmap;

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
mod input;
mod bodyparts;

use ecs::*;
use shrev::EventChannel;
use tcod::input::*;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 33;

pub const VIEWPORT_WIDTH: i32 = SCREEN_WIDTH;
pub const VIEWPORT_HEIGHT: i32 = SCREEN_HEIGHT - 8;

pub const VIEWPORT_POS_X: i32 = 0;
pub const VIEWPORT_POS_Y: i32 = 0;

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 25;

pub const BASE_TURN_TIME: u32 = 1000;
pub const MIN_TURN_TIME: u32 = 1;


fn main() {
    let mut debug = false;
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("debug")) { debug = true; }
    let mut ecs= ecs::world_setup(debug);
    ecs.main_loop();
}