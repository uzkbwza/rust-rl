//use tcod::console::*;
//use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, MAP_WIDTH, MAP_HEIGHT, VIEWPORT_WIDTH, VIEWPORT_HEIGHT};
//use crate::systems::render::TileMap;
//use specs::shred::Fetch;
//use crate::components::*;
//
// pub fn render_viewport(ctx: &mut Rltk, pos: Position, renderable: Renderable) {
//     let (fg_r, fg_g, fg_b) = renderable.fg_color;
//     let (bg_r, bg_g, bg_b) = renderable.bg_color.unwrap();
//
//     let fg_color = RGB::from_u8(fg_r, fg_g, fg_b);
//     let bg_color = RGB::from_u8(bg_r, bg_g, bg_b);
//
//     ctx.print_color(
//         pos.x + 1,
//         pos.y + 1,
//         fg_color,
//         bg_color,
//         &renderable.glyph.to_string(),
//     );
// }
