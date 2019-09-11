use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use tcod::console::*;
use tcod::colors;
use rand::prelude::*;
use crate::components::{Position, Floor, OnFloor, Renderable};
use crate::map::EntityMap;
use crate::systems::movement::MoveEvent;


#[derive(SystemData)]
pub struct RenderData<'a> {
        renderables: ReadStorage<'a, Renderable>,
        positions: ReadStorage<'a, Position>,
        floors:    ReadStorage<'a, Floor>,
        on_floors:    ReadStorage<'a, OnFloor>,
        root:        WriteExpect<'a, Root>,
        game_state: ReadExpect<'a, crate::GameState>,
        entity_map: Write<'a, EntityMap>,
        move_event_channel: Read<'a, EventChannel<MoveEvent>>,
}

pub struct Render {
    move_event_reader: Option<ReaderId<MoveEvent>>,
    initialized: bool
}

impl Render {
    pub fn new() -> Self {
        Render {
            move_event_reader: None,
            initialized: false
        }
    }
}

impl<'a> System<'a> for Render {
    type SystemData = RenderData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        data.root.clear();
        // render floors first... 
        if data.game_state.player_turn {
            for (rend, pos, floor) in (&data.renderables, &data.positions, &data.floors).join() {
                data.root.put_char(pos.x, pos.y, rend.glyph, BackgroundFlag::None);
                data.root.set_char_foreground(pos.x, pos.y, rend.color);
            }        
            
            // ...then things on top of the floor...
            for (rend, pos, on_floor) in (&data.renderables, &data.positions, &data.on_floors).join() {
                data.root.put_char(pos.x, pos.y, rend.glyph, BackgroundFlag::None);
                data.root.set_char_foreground(pos.x, pos.y, rend.color);
            }

            // ...then everything else.
            for (rend, pos, _on_floor, _floor) in (&data.renderables, &data.positions, !&data.floors, !&data.on_floors).join() {
                data.root.put_char(pos.x, pos.y, rend.glyph, BackgroundFlag::None);
                data.root.set_char_foreground(pos.x, pos.y, rend.color);
            }

            data.root.flush();
        }

        // premature optimization below. complicates stuff. just dont bother

        // }
        // self.initialized = true;

        // if data.game_state.player_turn {
        //     tcod::system::set_fps(60);
        //     let move_events = data.move_event_channel
        //         .read(self.move_event_reader
        //         .as_mut()
        //         .unwrap());
                
        //     for move_event in move_events {  
        //         let ent = move_event.entity;
        //         if let Some(rend) = data.renderables.get(ent) {
        //             let start_x = move_event.start_x;
        //             let start_y = move_event.start_y;
        //             let dest_x = move_event.dest_x;
        //             let dest_y = move_event.dest_y;

        //             data.entity_map.renderables
        //                 .remove(&(start_x, start_y));

        //             data.root.put_char(start_x, start_y, ' ', BackgroundFlag::None);
        //             data.root.set_char_foreground(start_x, start_y, colors::WHITE);

        //             data.entity_map.renderables
        //                 .insert((dest_x, dest_y), ent);

        //             data.root.put_char(dest_x, dest_y, rend.glyph, BackgroundFlag::None);
        //             data.root.set_char_foreground(dest_x, dest_y, rend.color);
        //         }
        //     }
        // } 
        // data.root.flush();
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.move_event_reader = Some(world.
            fetch_mut::<EventChannel<MoveEvent>>()
            .register_reader());
    }
}