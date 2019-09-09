use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use tcod::console::*;
use tcod::colors;
use crate::components::{Renderable};
use crate::map::EntityMap;
use crate::systems::movement::MoveEvent;

#[derive(SystemData)]
pub struct RenderData<'a> {
        renderables: ReadStorage<'a, Renderable>,
        root:        WriteExpect<'a, Root>,
        game_state: ReadExpect<'a, crate::GameState>,
        entity_map: Write<'a, EntityMap>,
        move_event_channel: Read<'a, EventChannel<MoveEvent>>,
}

pub struct Render {
    move_event_reader: Option<ReaderId<MoveEvent>>,
}

impl Render {
    pub fn new() -> Self {
        Render {
            move_event_reader: None
        }
    }
}

impl<'a> System<'a> for Render {
    type SystemData = RenderData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if data.game_state.player_turn {
            tcod::system::set_fps(60);
            let move_events = data.move_event_channel
                .read(self.move_event_reader
                .as_mut()
                .unwrap());
                
            for move_event in move_events {  
                let ent = move_event.entity;
                if let Some(rend) = data.renderables.get(ent) {
                    let start_x = move_event.start_x;
                    let start_y = move_event.start_y;
                    let dest_x = move_event.dest_x;
                    let dest_y = move_event.dest_y;

                    data.entity_map.renderables
                        .remove(&(start_x, start_y));

                    data.root.put_char(start_x, start_y, ' ', BackgroundFlag::None);
                    data.root.set_char_foreground(start_x, start_y, colors::WHITE);

                    data.entity_map.renderables
                        .insert((dest_x, dest_y), ent);

                    data.root.put_char(dest_x, dest_y, rend.glyph, BackgroundFlag::None);
                    data.root.set_char_foreground(dest_x, dest_y, rend.color);
                }
            }
        } 
        else { 
            tcod::system::set_fps(100000) 
        }
        data.root.flush();
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.move_event_reader = Some(world.
            fetch_mut::<EventChannel<MoveEvent>>()
            .register_reader());
    }
}