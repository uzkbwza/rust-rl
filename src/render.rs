use rltk::{Console, GameState, Rltk, VirtualKeyCode, RGB, Tile};
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, MAP_WIDTH, MAP_HEIGHT, VIEWPORT_WIDTH, VIEWPORT_HEIGHT};
use crate::systems::render::TileMap;
use specs::shred::Fetch;

pub fn render_viewport(ctx: &mut Rltk, tilemap: Fetch<TileMap>) {
    for entry in tilemap.elements_column_major_iter() {
        if let Some(t) = entry {
            if t.position.x < 0 || t.position.y < 0 || t.bg_color == None {
                continue
            }
            ctx.print_color(
                t.position.x + 1,
                t.position.y + 1,
                t.fg_color,
                t.bg_color.unwrap(),
                &t.glyph.to_string(),
            );
        }
    }
}

rltk::embedded_resource!(TILE_FONT, "../term.jpg");
pub fn make_window() -> Rltk {
    let mut window = Rltk::init_raw(SCREEN_WIDTH as u32 * 16, SCREEN_HEIGHT as u32 * 16, "RLTK");
    rltk::link_resource!(TILE_FONT, "term.jpg");
    let font = window.register_font(rltk::Font::load("term.jpg", (16, 16)));
    window.register_console(rltk::SimpleConsole::init(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, &window.gl), font, );
    window
}