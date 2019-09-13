use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use tcod::console::*;
use tcod::colors;
use crate::components::{Position, Floor, OnFloor, Renderable, Camera, Actor};
use crate::systems::movement::MoveEvent;
use crate::map::EntityMap;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, MAP_WIDTH, MAP_HEIGHT};


#[derive(SystemData)]
pub struct RenderData<'a> {
        renderables: ReadStorage<'a, Renderable>,
        positions: ReadStorage<'a, Position>,
        actors: ReadStorage<'a, Actor>,
        cameras: ReadStorage<'a, Camera>,
        floors:    ReadStorage<'a, Floor>,
        on_floors:    ReadStorage<'a, OnFloor>,
        root:        WriteExpect<'a, Root>,
        game_state: ReadExpect<'a, crate::GameState>,
        _entity_map: ReadExpect<'a, EntityMap>,
}

pub struct Render {
    move_event_reader: Option<ReaderId<MoveEvent>>,
}

impl Render {
    pub fn new() -> Self {
        Render {
            move_event_reader: None,
        }
    }
    pub fn get_screen_coordinates(pos: &Position, camera_pos: (i32, i32)) -> (i32, i32) {
        let screen_center = (SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        (screen_center.0 + (pos.x - camera_pos.0), screen_center.1 + (pos.y - camera_pos.1))
    }

    pub fn render(camera_pos: (i32, i32), glyph: char, color: colors::Color, pos: &Position, root: &mut Root) {
        let rend_pos = Self::get_screen_coordinates(pos, camera_pos);
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

        for (pos, _camera) in (&data.positions, &data.cameras).join() {
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

        if data.game_state.player_turn {
            data.root.clear();

            // render floors first...
            tcod::system::set_fps(60);
            for (rend, pos, _floor) in (&data.renderables, &data.positions, &data.floors).join() {
                Self::render(camera_position, rend.glyph, rend.color, &pos, &mut data.root);
            }        
            
            // ...then things on top of the floor...
            for (rend, pos, _on_floor) in (&data.renderables, &data.positions, &data.on_floors).join() {
                Self::render(camera_position, rend.glyph, rend.color, &pos, &mut data.root);
            }

            // ...then everything else.
            for (rend, pos, _on_floor, _floor) in (&data.renderables, &data.positions, !&data.floors, !&data.on_floors).join() {
                Self::render(camera_position, rend.glyph, rend.color, &pos, &mut data.root);
            }

            if data.game_state.debug {
                for (_rend, pos, actor) in (&data.renderables, &data.positions, &data.actors).join() {
                    let rend_pos = Self::get_screen_coordinates(pos, camera_position);
                    if rend_pos.0 - 1 < 0 || rend_pos.0 >= SCREEN_WIDTH { continue };
                    if rend_pos.1 < 0 || rend_pos.1 >= SCREEN_HEIGHT { continue };
                    data.root.print(rend_pos.0 - 1, rend_pos.1 + 1, format!("{}", actor.fatigue));
                }
            }

        } else {
            // not sure WHY i have to do this... but locking the FPS seems to slow the whole game down. 
            // chalking that up to another weird tcod thing.
            tcod::system::set_fps(1000000);
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