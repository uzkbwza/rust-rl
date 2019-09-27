use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use tcod::console::*;
use tcod::colors;
use tcod::input;
use crate::components::*;
use crate::MessageLog;
use crate::map::{EntityMap, View};
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, MAP_WIDTH, MAP_HEIGHT};
use tcod::map::FovAlgorithm;

trait Screen {
    fn render(&self, data: &mut RenderData);
}

struct Viewport {
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

impl Viewport {
    pub fn render_char(&self, camera_pos: (i32, i32), glyph: char, fg_color: colors::Color, bg_color: Option<colors::Color>, pos: &Position, root: &mut Root, map: &tcod::Map) {
        let rend_pos = self.get_screen_coordinates(pos, camera_pos);
        if rend_pos.0 < 0 || rend_pos.0 >= self.width { return };
        if rend_pos.1 < 0 || rend_pos.1 >= self.height { return };
        root.put_char(rend_pos.0 + self.x, rend_pos.1 + self.y, glyph, BackgroundFlag::Set);
        root.set_char_foreground(rend_pos.0 + self.x, rend_pos.1 + self.y, fg_color);
        if let Some(bg) = bg_color {
            root.set_char_background(rend_pos.0 + self.x, rend_pos.1 + self.y, bg, BackgroundFlag::Add);
        }
    }

    pub fn get_screen_coordinates(&self, pos: &Position, camera_pos: (i32, i32)) -> (i32, i32) {
        let screen_center = (self.width / 2, self.height / 2);
        (screen_center.0 + pos.x - camera_pos.0, screen_center.1 + pos.y - camera_pos.1)
    }
    
    pub fn is_on_screen(&self, coords: (i32, i32)) -> bool {
        if coords.0 >= 0 && coords.0 <= self.width && coords.1 >= 0 && coords.1 <= self.height {
            return true
        }
        false
    }

    pub fn get_world_coordinates(&self, rend_pos: (i32, i32), camera_pos: (i32, i32)) -> (i32, i32) {
        let screen_center = (self.width / 2, self.height / 2);
        let mut wx = rend_pos.0 - screen_center.0 + camera_pos.0;
        let mut wy = rend_pos.1 - screen_center.1 + camera_pos.1;    
        if wx > MAP_WIDTH { wx = MAP_WIDTH }    
        if wy > MAP_HEIGHT { wy = MAP_HEIGHT }    
        (wx, wy)
    }
}

impl Screen for Viewport {
    fn render(&self, data: &mut RenderData) {
        // determine camera position
        let mut camera_position = (0, 0);
        let viewport_width = self.width;
        let viewport_height = self.height;
        if viewport_width < MAP_WIDTH {
            for (pos, _camera) in (&data.positions, &data.cameras).join() {
                camera_position.0 = pos.x;
                if camera_position.0 - viewport_width / 2 < 0 {
                    camera_position.0 = viewport_width / 2;
                } else if camera_position.0 + viewport_width / 2 > MAP_WIDTH {
                    camera_position.0 = MAP_WIDTH - viewport_width / 2
                }
            }
        } else { 
            camera_position.0 = viewport_width / 2 
            }
        
        if viewport_height < MAP_HEIGHT {
            for (pos, _camera) in (&data.positions, &data.cameras).join() {
                camera_position.1 = pos.y;
                if camera_position.1 - viewport_height / 2 < 1 {
                    camera_position.1 = viewport_height / 2;
                } else if camera_position.1 + viewport_height / 2 > MAP_HEIGHT {
                    camera_position.1 = MAP_HEIGHT - viewport_height / 2
                }
            }
        } else { camera_position.1 = viewport_height / 2 }

        let mut fov_map = &mut data.view.map.lock().unwrap();
        
        for (ent, player, seer, pos, actor) in (&data.entities, &data.players, &mut data.seers, &data.positions, &data.actors).join() {
            if data.game_state.player_turn || data.game_state.debug {
                data.root.clear();
                let map = &data.entity_map;
                let radius = seer.fov;
                let seen_fg_color = colors::Color::new(10, 10, 15);
                let seen_bg_color = colors::BLACK;
                fov_map.compute_fov(pos.x, pos.y, radius, true, FovAlgorithm::Basic);

                // render floors first...
               
                for (rend, pos, _floor) in (&data.renderables, &data.positions, &data.floors).join() {
                    if fov_map.is_in_fov(pos.x, pos.y) {
                        self.render_char(camera_position, rend.glyph, rend.fg_color, rend.bg_color, &pos, &mut data.root, fov_map);    
                        seer.seen.insert((pos.x, pos.y), rend.glyph);
                    } else if seer.seen.contains_key(&(pos.x, pos.y)) {
                        if let Some(glyph) = seer.seen.get(&(pos.x, pos.y)) {                        
                            self.render_char(camera_position, *glyph, seen_fg_color, Some(seen_bg_color), &pos, &mut data.root, fov_map);
                        }
                    }
                }
                
                // ...then things on top of the floor...
                for (rend, pos, _on_floor) in (&data.renderables, &data.positions, &data.on_floors).join() {
                        if fov_map.is_in_fov(pos.x, pos.y) {
                            self.render_char(camera_position, rend.glyph, rend.fg_color, rend.bg_color, &pos, &mut data.root, fov_map);
                            seer.seen.insert((pos.x, pos.y), rend.glyph);
                        } else if seer.seen.contains_key(&(pos.x, pos.y)) {
                            if let Some(glyph) = seer.seen.get(&(pos.x, pos.y)) {                        
                                self.render_char(camera_position, *glyph, seen_fg_color, Some(seen_bg_color), &pos, &mut data.root, fov_map);
                            }
                        }
                    }

                // ...then everything else.
                for (ent, rend, pos, _on_floor, _floor) in (&data.entities, &data.renderables, &data.positions, !&data.floors, !&data.on_floors).join() {
                    if fov_map.is_in_fov(pos.x, pos.y) {
                        self.render_char(camera_position, rend.glyph, rend.fg_color, rend.bg_color, &pos, &mut data.root, fov_map);
                        if &data.actors.get(ent) == &Option::<&Actor>::None { 
                            seer.seen.insert((pos.x, pos.y), rend.glyph); 
                        }

                    } else if seer.seen.contains_key(&(pos.x, pos.y)) {
                        if let Some(glyph) = seer.seen.get(&(pos.x, pos.y)) {                        
                            self.render_char(camera_position, *glyph, seen_fg_color, Some(seen_bg_color), &pos, &mut data.root, fov_map);
                        }
                    }              
                }

                if data.game_state.debug {
                    data.root.print(0, 0, format!("{}, {}", pos.x, pos.y));
                    data.root.print(0, 1, format!("{}, {}", camera_position.0, camera_position.1));
                    for x in 0..viewport_width {
                        for y in 0..viewport_height {
                            let mut world_pos = self.get_world_coordinates((x, y), camera_position);

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
                                data.root.set_char_background(x + self.x, y + self.y, colors::DARKEST_ORANGE, BackgroundFlag::Set);
                            }
                        }
                    }

                    for (ent, _rend, pos, actor, corporeal) in (&data.entities, &data.renderables, &data.positions, &data.actors, &data.corporeals).join() {
                        let rend_pos = self.get_screen_coordinates(pos, camera_position);
                        if rend_pos.0 < 0 || rend_pos.0 >= viewport_width { continue };
                        if rend_pos.1 < 0 || rend_pos.1 >= viewport_height { continue };
                        if let Some(_turn) = &data.my_turns.get(ent) {
                            data.root.set_char_background(rend_pos.0 + self.x, rend_pos.1 + self.y, colors::GREEN, BackgroundFlag::Set);
                        }
                        // data.root.print(rend_pos.0, rend_pos.1+1, format!("{}", actor.fatigue));
                        // data.root.print(rend_pos.0, rend_pos.1+1, format!("{}/{}", corporeal.hp, corporeal.max_hp));
                    }
                }
            }
        }
        if data.game_state.debug {
            for (ai, pos, seer) in (&data.ais, &data.positions, &mut data.seers).join() {
                fov_map.compute_fov(pos.x, pos.y, seer.fov, true, FovAlgorithm::Basic);
                for x in 0..i32::min(viewport_width, MAP_WIDTH) {
                    for y in 0..i32::min(viewport_height, MAP_HEIGHT) {
                        let world_coords = self.get_world_coordinates((x, y), camera_position);
                        if fov_map.is_in_fov(world_coords.0, world_coords.1) {
                            let mut bg = data.root.get_char_background(x + self.x, y + self.y);
                            if bg.r <= 250 {
                                bg.r += 5
                            } else {
                                bg.r = 255
                            }
                            data.root.set_char_background(x + self.x, y + self.y, bg, BackgroundFlag::Set);
                        }
                    }
                }
            }
            data.root.print(0, 2, format!("World tick: {}", data.game_state.world_time.tick));
            data.root.print(0, 3, format!("World turns: {}", data.game_state.world_time.world_turns));
            data.root.print(0, 4, format!("Player turns: {}", data.game_state.world_time.player_turns));
            for (player, actor, ent) in (&data.players, &data.actors, &data.entities).join() {
                data.root.print(0, 5, format!("Player next turn: {}", actor.next_turn));
            }
            data.root.print(0, 6, format!("player turn: {}", data.game_state.player_turn));
            // data.root.print(0, 7, format!("next turn: {:?} at {}", data.next_turn.entity, data.next_turn.tick));
        }
    }
}

#[derive(SystemData)]
pub struct RenderData<'a> {
        entities: Entities<'a>,
        corporeals: ReadStorage<'a, Corporeal>,
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
        message_log: WriteExpect<'a, MessageLog>,
        // turn_queue: WriteExpect<'a, crate::TurnQueue>,
}

pub struct Render {
    screens: Vec<Box<dyn Screen>>
}

impl Render {

    pub fn new() -> Self {
        let mut screens: Vec<Box<dyn Screen>> = Vec::new();
        let viewport = Viewport {
            x: 1, 
            y: 1, 
            width: SCREEN_WIDTH - 31,
            height: SCREEN_HEIGHT - 2
            };
            
        screens.push(Box::new(viewport));
        Render { screens }

    }
}

impl<'a> System<'a> for Render {
    type SystemData = RenderData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // tcod::system::set_fps(4);
        for screen in &self.screens {
            screen.render(&mut data)
        }
        // tcod::system::set_fps(60);
        // println!("{:?}", tcod::system::get_fps());

        let box_height: i32 = data.message_log.capacity as i32;

        let mut loglines = String::new();
        if data.message_log.messages.len() > 0 {
            for (i, message) in data.message_log.messages.iter().enumerate() {
                loglines = format!("{}\n{}", message, loglines);
            }
        }

        if data.message_log.messages.len() < box_height as usize {
            let empty_lines = "\n".repeat(box_height as usize - data.message_log.messages.len());
            loglines = format!("{}{}", empty_lines, loglines);
        }
        
        data.root.print_rect(SCREEN_WIDTH - 29, SCREEN_HEIGHT - box_height - 1, 30, box_height, loglines);
        data.root.flush();
    } 
}