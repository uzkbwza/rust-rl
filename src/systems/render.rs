use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use tcod::console::*;
use tcod::colors;
use crate::components::{PlayerControl, Position, Floor, OnFloor, Renderable, Camera, Name, Actor, Seeing, MyTurn};
use crate::systems::movement::MoveEvent;
use crate::map::{EntityMap, View};
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, MAP_WIDTH, MAP_HEIGHT};
use tcod::map::FovAlgorithm;


#[derive(SystemData)]
pub struct RenderData<'a> {
        entities: Entities<'a>,
        renderables: ReadStorage<'a, Renderable>,
        my_turns: ReadStorage<'a, MyTurn>,
        names: ReadStorage<'a, Name>,
        positions: ReadStorage<'a, Position>,
        players: ReadStorage<'a, PlayerControl>,
        actors: ReadStorage<'a, Actor>,
        cameras: ReadStorage<'a, Camera>,
        floors:    ReadStorage<'a, Floor>,
        on_floors:    ReadStorage<'a, OnFloor>,
        root:        WriteExpect<'a, Root>,
        game_state: ReadExpect<'a, crate::GameState>,
        entity_map: ReadExpect<'a, EntityMap>,
        view: ReadExpect<'a, View>,
        seers: WriteStorage<'a, Seeing>,
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
        (
            screen_center.0 + pos.x - camera_pos.0, 
            screen_center.1 + pos.y - camera_pos.1
            )
    }

    pub fn get_world_coordinates(rend_pos: (i32, i32), camera_pos: (i32, i32)) -> (i32, i32) {
        let screen_center = (SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        
        (
            rend_pos.0 - screen_center.0 + camera_pos.0,
            rend_pos.1 - screen_center.1 + camera_pos.1,
            )
    }

    pub fn render(camera_pos: (i32, i32), glyph: char, color: colors::Color, pos: &Position, root: &mut Root, map: &tcod::Map) {
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

        for (ent, player, seer, pos) in (&data.entities, &data.players, &mut data.seers, &data.positions).join() {
            // if data.game_state.player_turn {
                data.root.clear();
                let mut fov_map = &mut data.view.map.lock().unwrap();
                let map = &data.entity_map;
                let radius = seer.fov;
                fov_map.compute_fov(pos.x, pos.y, radius, true, FovAlgorithm::Basic);

                // render floors first...
                tcod::system::set_fps(60);
                for (rend, pos, _floor) in (&data.renderables, &data.positions, &data.floors).join() {
                    if fov_map.is_in_fov(pos.x, pos.y) {
                        Self::render(camera_position, rend.glyph, rend.color, &pos, &mut data.root, fov_map);
                        seer.seen.insert((pos.x, pos.y), rend.glyph);
                    } else if seer.seen.contains_key(&(pos.x, pos.y)) {
                        // println!("true");
                        Self::render(camera_position, *seer.seen.get(&(pos.x, pos.y)).unwrap(), colors::Color::new(10, 10, 15), &pos, &mut data.root, fov_map);
                    }
                }
                
                // ...then things on top of the floor...
                for (rend, pos, _on_floor) in (&data.renderables, &data.positions, &data.on_floors).join() {
                    if fov_map.is_in_fov(pos.x, pos.y) {
                        Self::render(camera_position, rend.glyph, rend.color, &pos, &mut data.root, fov_map);
                        seer.seen.insert((pos.x, pos.y), rend.glyph);

                    } else if seer.seen.contains_key(&(pos.x, pos.y)) {
                        Self::render(camera_position, *seer.seen.get(&(pos.x, pos.y)).unwrap(), colors::Color::new(10,10,15), &pos, &mut data.root, fov_map);
                    }
                }

                // ...then everything else.
                for (ent, rend, pos, _on_floor, _floor) in (&data.entities, &data.renderables, &data.positions, !&data.floors, !&data.on_floors).join() {
                    if fov_map.is_in_fov(pos.x, pos.y) {
                        Self::render(camera_position, rend.glyph, rend.color, &pos, &mut data.root, fov_map);
                        if let Some(actor) = &data.actors.get(ent) {} else { seer.seen.insert((pos.x, pos.y), rend.glyph); }
                    } else if seer.seen.contains_key(&(pos.x, pos.y)) {
                        Self::render(camera_position, *seer.seen.get(&(pos.x, pos.y)).unwrap(), colors::Color::new(10,10,15), &pos, &mut data.root, fov_map);
                    }                
                }

                if data.game_state.debug {
                    for x in 0..SCREEN_WIDTH {
                        for y in 0..SCREEN_HEIGHT {
                            let world_pos = Self::get_world_coordinates((x, y), camera_position);
                            if fov_map.is_walkable(world_pos.0, world_pos.1) {
                                data.root.set_char_background(x, y, colors::Color::new(30,0,0), BackgroundFlag::Set);
                            }
                            if let Some(_ent) = map.actors.get(&world_pos) {
                                data.root.set_char_background(x, y, colors::ORANGE, BackgroundFlag::Set);
                            }
                        }
                    }

                    for (ent, _rend, pos, actor, name) in (&data.entities, &data.renderables, &data.positions, &data.actors, &data.names).join() {
                        let rend_pos = Self::get_screen_coordinates(pos, camera_position);
                        if rend_pos.0 < 0 || rend_pos.0 >= SCREEN_WIDTH { continue };
                        if rend_pos.1 < 0 || rend_pos.1 >= SCREEN_HEIGHT { continue };
                        data.root.print(rend_pos.0, rend_pos.1 + 1, format!("{}\n{}\nscreen center: {}, {}\nworld pos: {}, {}\nrend pos: {}, {}\ncamera pos: {}, {}", name.name, actor.fatigue, SCREEN_WIDTH/2, SCREEN_HEIGHT/2, pos.x, pos.y, rend_pos.0, rend_pos.1, camera_position.0, camera_position.1));
                        if let Some(_turn) = &data.my_turns.get(ent) {
                            data.root.set_char_background(rend_pos.0, rend_pos.1, colors::WHITE, BackgroundFlag::Set);
                        }
                    }
                }

            // } else {
            //     // not sure WHY i have to do this... but locking the FPS seems to slow the whole game down. 
            //     // chalking that up to another weird tcod thing.
            //     tcod::system::set_fps(1000000);
            // }
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