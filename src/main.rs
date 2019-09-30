extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate shrev;
extern crate shred;
extern crate shred_derive;
extern crate rltk;

use rltk::{
    Algorithm2D, BaseMap, Console, DistanceAlg, GameState, Point, Rltk, VirtualKeyCode, RGB,
};

use specs::prelude::*;
use std::env;

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
use crate::systems::render::TileMap;
use crate::systems::movement::CollisionMapUpdater;
use specs::shred::Fetch;
use shrev::EventChannel;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
pub const VIEWPORT_WIDTH: i32 = SCREEN_WIDTH - 31;
pub const VIEWPORT_HEIGHT: i32 = SCREEN_HEIGHT - 2;
pub const MAP_WIDTH: i32 = 100;
pub const MAP_HEIGHT: i32 = 100;
pub const BASE_TURN_TIME: u32 = 1000;
pub const MIN_TURN_TIME: u32 = 1;

struct State {
    world: World,
    dispatcher: Dispatcher<'static, 'static>
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        self.world.maintain();
        self.dispatcher.dispatch(&mut self.world);
        ctx.cls();
        render::render_viewport(ctx, self.world.read_resource::<TileMap>());
        input::send(ctx, self.world.write_resource::<EventChannel<VirtualKeyCode>>());
    }
}

fn main() {
    let mut DEBUG = false;
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("debug")) { DEBUG = true; }
    let (mut world, mut dispatcher) = ecs::world_setup(DEBUG);
    let state = State{ world, dispatcher };
    let window = render::make_window();
    rltk::main_loop(window, state);
}