use specs::prelude::*;
use tcod::console::*;
use crate::components::*;

#[derive(SystemData)]
pub struct RenderData<'a> {
        renderables: ReadStorage<'a, Renderable>,
        positions:  ReadStorage<'a, Position>,
        root:        WriteExpect<'a, Root>,
}

pub struct Render;
impl<'a> System<'a> for Render {
    type SystemData = RenderData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        data.root.clear();
        for (rend, pos) in (&data.renderables, &data.positions).join() {
            data.root.put_char(pos.x, pos.y, rend.glyph, BackgroundFlag::None)
        }
        data.root.flush();
    }
}