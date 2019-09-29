extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate shrev;
extern crate shred;
extern crate shred_derive;
extern crate rltk;

use rltk::{GameState, Rltk, Console, RGB};
use specs::prelude::*;
use std::env;

mod entities;
mod components;
mod map;
mod command;
mod time;
mod systems;
mod ecs;
use ecs::*;
use crate::systems::render::TileMap;
use crate::systems::movement::CollisionMapUpdater;
use specs::shred::Fetch;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
pub const VIEWPORT_WIDTH: i32 = SCREEN_WIDTH - 31;
pub const VIEWPORT_HEIGHT: i32 = SCREEN_HEIGHT - 2;
pub const MAP_WIDTH: i32 = 59;
pub const MAP_HEIGHT: i32 = 69;
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
        render(ctx, self.world.read_resource::<TileMap>());
    }
}

fn render(ctx: &mut Rltk, tilemap: Fetch<TileMap>) {
    for entry in tilemap.elements_column_major_iter() {
        if let Some(t) = entry {
            if t.position.x < 0 || t.position.y < 0 {
                continue
            }
            ctx.print(t.position.x + 1, t.position.y + 1, &t.glyph.to_string());
        }
    }
}

fn main() {
    let mut DEBUG = false;
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("debug")) {
        DEBUG = true;
    }

    let (mut world, mut dispatcher) = ecs::world_setup(DEBUG);
//    dispatcher.dispatch(&world);

    let mut window = Rltk::init_simple8x8(
        SCREEN_WIDTH as u32,
        SCREEN_HEIGHT as u32,
        "RLTK",
        "resources"
    );

    let state = State{
        world,
        dispatcher
    };

    rltk::main_loop(window, state);
}