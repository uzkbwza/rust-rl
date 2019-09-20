use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use tcod::console::*;
use tcod::colors;
use tcod::input;
use crate::components::{AiControl, PlayerControl, Position, Floor, OnFloor, Renderable, Camera, Name, Actor, Seeing, MyTurn};
use crate::systems::movement::MoveEvent;
use crate::map::{EntityMap, View};
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, MAP_WIDTH, MAP_HEIGHT};
use tcod::map::FovAlgorithm;

pub enum RenderError {
    OutOfBounds
}

#[derive(SystemData)]
pub struct RenderData<'a> {
        entities: Entities<'a>,
        renderables: ReadStorage<'a, Renderable>,
        ais: ReadStorage<'a, AiControl>,
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
        (screen_center.0 + pos.x - camera_pos.0, screen_center.1 + pos.y - camera_pos.1)
    }
    
    pub fn is_on_screen(coords: (i32, i32)) -> bool {
        if coords.0 >= 0 && coords.0 <= SCREEN_WIDTH && coords.1 >= 0 && coords.1 <= SCREEN_HEIGHT {
            return true
        }
        false
    }

    pub fn get_world_coordinates(rend_pos: (i32, i32), camera_pos: (i32, i32)) -> (i32, i32) {
        let screen_center = (SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        let mut wx = rend_pos.0 - screen_center.0 + camera_pos.0;
        let mut wy = rend_pos.1 - screen_center.1 + camera_pos.1;    
        if wx > MAP_WIDTH { wx = MAP_WIDTH }    
        if wy > MAP_HEIGHT { wy = MAP_HEIGHT }    
        (wx, wy)
    }

    pub fn render(camera_pos: (i32, i32), glyph: char, fg_color: colors::Color, bg_color: Option<colors::Color>, pos: &Position, root: &mut Root, map: &tcod::Map) {
        let rend_pos = Self::get_screen_coordinates(pos, camera_pos);
        if rend_pos.0 < 0 || rend_pos.0 >= SCREEN_WIDTH { return };
        if rend_pos.1 < 0 || rend_pos.1 >= SCREEN_HEIGHT { return };
        root.put_char(rend_pos.0, rend_pos.1, glyph, BackgroundFlag::Set);
        root.set_char_foreground(rend_pos.0, rend_pos.1, fg_color);
        if let Some(bg) = bg_color {
            root.set_char_background(rend_pos.0, rend_pos.1, bg, BackgroundFlag::Set);
        }
    }
}

impl<'a> System<'a> for Render {
    type SystemData = RenderData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // determine camera position
        let mut camera_position = (0, 0);
        tcod::system::set_fps(60);

        if SCREEN_WIDTH < MAP_WIDTH {
            for (pos, _camera) in (&data.positions, &data.cameras).join() {
                camera_position.0 = pos.x;
                if camera_position.0 - SCREEN_WIDTH / 2 < 0 {
                    camera_position.0 = SCREEN_WIDTH / 2;
                } else if camera_position.0 + SCREEN_WIDTH / 2 > MAP_WIDTH {
                    camera_position.0 = MAP_WIDTH - SCREEN_WIDTH / 2
                }
            }
        } else { 
            camera_position.0 = SCREEN_WIDTH / 2 
            }
        
        if SCREEN_HEIGHT < MAP_HEIGHT {
            for (pos, _camera) in (&data.positions, &data.cameras).join() {
                camera_position.1 = pos.y;
                if camera_position.1 - SCREEN_HEIGHT / 2 < 1 {
                    camera_position.1 = SCREEN_HEIGHT / 2;
                } else if camera_position.1 + SCREEN_HEIGHT / 2 > MAP_HEIGHT {
                    camera_position.1 = MAP_HEIGHT - SCREEN_HEIGHT / 2
                }
            }
        } else { camera_position.1 = SCREEN_HEIGHT / 2 }

        let mut fov_map = &mut data.view.map.lock().unwrap();
        
        for (ent, player, seer, pos) in (&data.entities, &data.players, &mut data.seers, &data.positions).join() {
            if data.game_state.player_turn || data.game_state.debug {
                data.root.clear();
                let map = &data.entity_map;
                let radius = seer.fov;
                let seen_fg_color = colors::Color::new(10, 10, 15);
                let seen_bg_color = colors::Color::new(5, 5, 7);
                fov_map.compute_fov(pos.x, pos.y, radius, true, FovAlgorithm::Basic);

                // render floors first...
               
                for (rend, pos, _floor) in (&data.renderables, &data.positions, &data.floors).join() {
                    if fov_map.is_in_fov(pos.x, pos.y) {
                        Self::render(
                            camera_position, 
                            rend.glyph, 
                            rend.fg_color, 
                            rend.bg_color, 
                            &pos, 
                            &mut data.root, 
                            fov_map);
                            
                        seer.seen.insert((pos.x, pos.y), rend.glyph);

                    } else if seer.seen.contains_key(&(pos.x, pos.y)) {
                        if let Some(glyph) = seer.seen.get(&(pos.x, pos.y)) {                        
                            Self::render(
                                camera_position, 
                                *glyph, 
                                seen_fg_color, 
                                Some(seen_bg_color), 
                                &pos, 
                                &mut data.root, fov_map);
                        }
                    }
                }
                
                // ...then things on top of the floor...
                for (rend, pos, _on_floor) in (
                    &data.renderables, 
                    &data.positions, 
                    &data.on_floors)
                    .join() {
                        if fov_map.is_in_fov(pos.x, pos.y) {
                            Self::render(
                                camera_position, 
                                rend.glyph, 
                                rend.fg_color, 
                                rend.bg_color, 
                                &pos, 
                                &mut data.root, 
                                fov_map);
                                
                            seer.seen.insert((pos.x, pos.y), rend.glyph);

                        } else if seer.seen.contains_key(&(pos.x, pos.y)) {
                            if let Some(glyph) = seer.seen.get(&(pos.x, pos.y)) {                        
                                Self::render(
                                    camera_position, 
                                    *glyph, 
                                    seen_fg_color, 
                                    Some(seen_bg_color), 
                                    &pos, 
                                    &mut data.root, 
                                    fov_map);
                            }
                        }
                    }

                // ...then everything else.
                for (ent, rend, pos, _on_floor, _floor) in (&data.entities, &data.renderables, &data.positions, !&data.floors, !&data.on_floors).join() {
                    if fov_map.is_in_fov(pos.x, pos.y) {
                        Self::render(
                            camera_position, 
                            rend.glyph, 
                            rend.fg_color, 
                            rend.bg_color, 
                            &pos, 
                            &mut data.root, 
                            fov_map);

                        if &data.actors.get(ent) == &Option::<&Actor>::None { 
                            seer.seen.insert((pos.x, pos.y), rend.glyph); 
                        }
                    } else if seer.seen.contains_key(&(pos.x, pos.y)) {
                        if let Some(glyph) = seer.seen.get(&(pos.x, pos.y)) {                        
                            Self::render(
                                camera_position, 
                                *glyph, 
                                seen_fg_color, 
                                Some(seen_bg_color), 
                                &pos, 
                                &mut data.root, 
                                fov_map);
                        }
                    }              
                }

                if data.game_state.debug {
                    data.root.print(0, 0, format!("{}, {}", pos.x, pos.y));
                    data.root.print(0, 1, format!("{}, {}", camera_position.0, camera_position.1));
                    for x in 0..SCREEN_WIDTH {
                        for y in 0..SCREEN_HEIGHT {
                            let mut world_pos = Self::get_world_coordinates((x, y), camera_position);

                            match world_pos.0 {
                                x if x >= fov_map.size().0 => world_pos.0 = fov_map.size().0 - 1,
                                x if x < 0 => world_pos.0 = 0,
                                _ => (),
                            }
                            match world_pos.1 {
                                x if x >= fov_map.size().1 => world_pos.1 = fov_map.size().1 - 1,
                                x if x < 0 => world_pos.1 = 0,
                                _ => (),
                            }

                            if world_pos.1 >= fov_map.size().1 { world_pos.1 = fov_map.size().1 - 1}
                            if fov_map.is_walkable(world_pos.0, world_pos.1) {
                                // data.root.set_char_background(x, y, colors::Color::new(30,0,0), BackgroundFlag::Set);
                            }
                            if map.actors.contains_actor(world_pos.0, world_pos.1) {
                                data.root.set_char_background(x, y, colors::DARKEST_ORANGE, BackgroundFlag::Set);
                            }
                        }
                    }

                    for (ent, _rend, pos, _actor) in (&data.entities, &data.renderables, &data.positions, &data.actors).join() {
                        let rend_pos = Self::get_screen_coordinates(pos, camera_position);
                        if rend_pos.0 < 0 || rend_pos.0 >= SCREEN_WIDTH { continue };
                        if rend_pos.1 < 0 || rend_pos.1 >= SCREEN_HEIGHT { continue };
                        if let Some(_turn) = &data.my_turns.get(ent) {
                            data.root.set_char_background(rend_pos.0, rend_pos.1, colors::GREEN, BackgroundFlag::Set);
                        }
                    }
                }
            } else {
                // not sure WHY i have to do this... but locking the FPS seems to slow the whole game down. 
                // chalking that up to another weird tcod thing.
                tcod::system::set_fps(1000000);
            }
        }

        if data.game_state.debug {
            for (ai, pos, seer) in (&data.ais, &data.positions, &mut data.seers).join() {
                fov_map.compute_fov(pos.x, pos.y, seer.fov, true, FovAlgorithm::Basic);
                for x in 0..SCREEN_WIDTH {
                    for y in 0..SCREEN_HEIGHT {
                        let world_coords = Self::get_world_coordinates((x,y), camera_position);
                        if fov_map.is_in_fov(world_coords.0, world_coords.1) {
                            let mut bg = data.root.get_char_background(x, y);
                            if bg.r <= 200 {
                                bg.r += 55
                            } else {
                                bg.r = 255
                            }
                            data.root.set_char_background(x, y, bg, BackgroundFlag::Set);
                        }
                    }
                }
            }
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