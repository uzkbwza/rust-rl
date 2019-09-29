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
use ecs::*;
use crate::systems::render::TileMap;
use crate::systems::movement::CollisionMapUpdater;
use specs::shred::Fetch;

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
        render(ctx, self.world.read_resource::<TileMap>());
    }
}

fn render(ctx: &mut Rltk, tilemap: Fetch<TileMap>) {
    for entry in tilemap.elements_column_major_iter() {
        if let Some(t) = entry {
            if t.position.x < 0 || t.position.y < 0 || t.bg_color == None {
                continue
            }
            ctx.print_color(t.position.x + 1, t.position.y + 1, t.fg_color, t.bg_color.unwrap(), &t.glyph.to_string());
        }
    }
}
rltk::embedded_resource!(TILE_FONT, "../term.jpg");
fn main() {
    let mut DEBUG = false;
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("debug")) {
        DEBUG = true;
    }

    let (mut world, mut dispatcher) = ecs::world_setup(DEBUG);

    rltk::link_resource!(TILE_FONT, "term.jpg");

    let mut window = Rltk::init_raw(
        SCREEN_WIDTH as u32 * 16,
        SCREEN_HEIGHT as u32 * 16,
        "RLTK",
    );
    let font = window.register_font(rltk::Font::load("term.jpg", (16, 16)));
    window.register_console(
        rltk::SimpleConsole::init(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, &window.gl),
        font,
    );

    let state = State{
        world,
        dispatcher
    };

    rltk::main_loop(window, state);
}