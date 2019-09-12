use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use tcod::console::*;
use tcod::colors;
use rand::prelude::*;
use crate::components::{Position, Floor, OnFloor, Renderable, BoxRenderable, Camera, InView};
use crate::map::{View, EntityMap};
use crate::systems::movement::MoveEvent;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, MAP_WIDTH, MAP_HEIGHT};


#[derive(SystemData)]
pub struct RenderData<'a> {
        entities: Entities<'a>,
        renderables: ReadStorage<'a, Renderable>,
        box_renderables: ReadStorage<'a, BoxRenderable>,
        positions: ReadStorage<'a, Position>,
        cameras: ReadStorage<'a, Camera>,
        floors:    ReadStorage<'a, Floor>,
        on_floors:    ReadStorage<'a, OnFloor>,
        in_views:   WriteStorage<'a, InView>,
        root:        WriteExpect<'a, Root>,
        game_state: ReadExpect<'a, crate::GameState>,
        entity_map: Write<'a, EntityMap>,
        move_event_channel: Read<'a, EventChannel<MoveEvent>>,
        view: WriteExpect<'a, View>,
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
    pub fn get_screen_coordinates(pos: &Position, camera_pos: (i32, i32)) -> (i32, i32) {
        let screen_center = (SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        (screen_center.0 + (pos.x - camera_pos.0), screen_center.1 + (pos.y - camera_pos.1))
    }

    pub fn render(camera_pos: (i32, i32), glyph: char, color: colors::Color, pos: &Position, root: &mut Root) {
        let mut rend_pos = Self::get_screen_coordinates(pos, camera_pos);
        if rend_pos.0 < 0 || rend_pos.0 >= SCREEN_WIDTH { return };
        if rend_pos.1 < 0 || rend_pos.1 >= SCREEN_HEIGHT { return };
        root.put_char(rend_pos.0, rend_pos.1, glyph, BackgroundFlag::Set);
        root.set_char_foreground(rend_pos.0, rend_pos.1, color);
    }
}

impl<'a> System<'a> for Render {
    type SystemData = RenderData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // determine camera position
        let mut camera_position = (0, 0);

        for (pos, camera) in (&data.positions, &data.cameras).join() {
            camera_position = (pos.x, pos.y);
            if camera_position.0 - SCREEN_WIDTH / 2 < 0 {
                camera_position.0 = SCREEN_WIDTH / 2;
            } else if camera_position.0 + SCREEN_WIDTH / 2 > MAP_WIDTH {
                camera_position.0 = MAP_WIDTH - SCREEN_WIDTH / 2
            }

            if camera_position.1 - SCREEN_HEIGHT / 2 < 1 {
                camera_position.1 = SCREEN_HEIGHT / 2;
            } else if camera_position.1 + SCREEN_HEIGHT / 2 > MAP_HEIGHT {
                camera_position.1 = MAP_HEIGHT - SCREEN_HEIGHT / 2
            }
        }

        // render floors first...
        if data.game_state.player_turn {
            data.root.clear();
            // for x in 0..SCREEN_WIDTH {
            //     for y in 0..SCREEN_HEIGHT {
            //         data.root.put_char(x, y, '.',  BackgroundFlag::None);
            //         data.root.set_char_foreground(x, y, colors::DARKEST_GREY);

            //     }
            // }

            tcod::system::set_fps(60);
            for (rend, pos, _floor) in (&data.renderables, &data.positions, &data.floors).join() {
                Self::render(camera_position, rend.glyph, rend.color, &pos, &mut data.root);
            }        
            
            // ...then things on top of the floor...
            for (rend, pos, on_floor) in (&data.renderables, &data.positions, &data.on_floors).join() {
                Self::render(camera_position, rend.glyph, rend.color, &pos, &mut data.root);
            }

            // ...then everything else.
            for (rend, pos, _on_floor, _floor) in (&data.renderables, &data.positions, !&data.floors, !&data.on_floors).join() {
                Self::render(camera_position, rend.glyph, rend.color, &pos, &mut data.root);
            }
        } else {
            // not sure WHY i have to do this... but locking the FPS seems to slow the whole game down. 
            // chalking that up to another weird tcod thing.
            tcod::system::set_fps(10000);
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