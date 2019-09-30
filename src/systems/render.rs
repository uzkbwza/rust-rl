use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use rltk::{Console, GameState, Rltk, VirtualKeyCode, RGB};
use crate::components::*;
use crate::ecs::MessageLog;
use crate::map::{EntityMap, View};
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, MAP_WIDTH, MAP_HEIGHT};
use tcod::map::FovAlgorithm;
use array2d::Array2D;
use std::process::exit;

pub type TileMap = Array2D<Option<Tile>>;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum Elevation {
    Floor,
    OnFloor,
    Upright,
    _InAir,
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub position: Position,
    pub elevation: Elevation,
    pub glyph: char,
    pub fg_color: RGB,
    pub bg_color: Option<RGB>,
}

impl Tile {
    pub fn new() -> Self {
        Tile {
            position: Position::new(-1, -1),
            elevation: Elevation::Floor,
            glyph: ' ',
            fg_color: RGB::from_u8(255, 255, 255),
            bg_color: Some(RGB::from_u8(0, 0, 0))
        }
    }
}

struct Viewport {
    width: i32,
    height: i32,
}

impl Viewport {

    fn update(&self, data: &mut RenderData) { Self::set_map(&self, data) }

    pub fn set_tile(&self, pos: Position, camera_pos: Position, mut tile: Tile, tile_map: &mut TileMap) {
        let rend_pos = self.get_screen_coordinates(pos, camera_pos);
        let (x, y) = (rend_pos.x, rend_pos.y);
        if x < 0 || x >= self.width { return };
        if y < 0 || y >= self.height { return };
        if let Some(Some(existing_tile)) = tile_map.get(x as usize, y as usize) {
            if tile.elevation < existing_tile.elevation {
                return
            }
            if let Some(bg_color) = existing_tile.bg_color {
                if tile.bg_color == None {
                    tile.bg_color = existing_tile.bg_color
                }
            } else {
                if tile.bg_color == None {
                    tile.bg_color = Some(RGB::from_u8(0, 0, 0))
                }
            }
        }

        tile_map[(x as usize, y as usize)] = Some(tile);
    }

    // creates full character map of what the player sees.
    fn set_map(&self, mut data: &mut RenderData) {

        let mut tile_map = TileMap::filled_with(None, MAP_WIDTH as usize, MAP_HEIGHT as usize);
        let camera_pos = self.get_camera_position(data);
        for (ent, pos, renderable) in (&data.entities, &data.positions, &data.renderables).join() {
            let (glyph, fg_color, bg_color) = (renderable.glyph, renderable.fg_color, renderable.bg_color);
            let mut elevation = Elevation::Upright;
            if let Some(_) = data.floors.get(ent) {
                elevation = Elevation::Floor
            }
            if let Some(_) = data.on_floors.get(ent) {
                elevation = Elevation::OnFloor
            }

            let mut tile = Tile {
                position: self.get_screen_coordinates(*pos, camera_pos),
                elevation,
                glyph,
                fg_color,
                bg_color,
            };

            let mut fov_map = data.view.map.lock().unwrap();
            if !fov_map.is_in_fov(pos.x, pos.y) {
                tile = Tile::new();
            }
            self.set_tile(*pos, camera_pos, tile, &mut tile_map);
        }
        *data.tile_map = tile_map;
    }

    fn get_screen_coordinates(&self, pos: Position, camera_pos: Position) -> Position {
        let screen_center = Position::new(self.width / 2, self.height / 2);
        Position::new(screen_center.x + pos.x - camera_pos.x, screen_center.y + pos.y - camera_pos.y)
    }

    fn is_on_screen(&self, coords: Position) -> bool {
        if coords.x >= 0 && coords.x <= self.width && coords.y >= 0 && coords.y <= self.height {
            return true
        }
        false
    }

    fn get_world_coordinates(&self, rend_pos: Position, camera_pos: Position) -> Position {
        let screen_center = Position::new(self.width / 2, self.height / 2);
        let mut wx = rend_pos.x - screen_center.x + camera_pos.x;
        let mut wy = rend_pos.x - screen_center.x + camera_pos.y;
        if wx > MAP_WIDTH { wx = MAP_WIDTH }
        if wy > MAP_HEIGHT { wy = MAP_HEIGHT }
        Position::new(wx, wy)
    }

    fn get_camera_position(&self,  data: &RenderData) -> Position {
        let mut camera_position = Position::new(0, 0);
        let viewport_width = self.width;
        let viewport_height = self.height;
        if viewport_width < MAP_WIDTH {
            for (pos, _camera) in (&data.positions, &data.cameras).join() {
                camera_position.x = pos.x;
                if camera_position.x - viewport_width / 2 < 0 {
                    camera_position.x = viewport_width / 2;
                } else if camera_position.x + viewport_width / 2 > MAP_WIDTH {
                    camera_position.x = MAP_WIDTH - viewport_width / 2
                }
            }
        } else {
            camera_position.x = viewport_width / 2
        }

        if viewport_height < MAP_HEIGHT {
            for (pos, _camera) in (&data.positions, &data.cameras).join() {
                camera_position.y = pos.y;
                if camera_position.y - viewport_height / 2 < 1 {
                    camera_position.y = viewport_height / 2;
                } else if camera_position.y + viewport_height / 2 > MAP_HEIGHT {
                    camera_position.y = MAP_HEIGHT - viewport_height / 2
                }
            }
        } else {
            camera_position.y = viewport_height / 2
        }
        camera_position
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
        world_resources: ReadExpect<'a, crate::WorldResources>,
        entity_map: ReadExpect<'a, EntityMap>,
        view: ReadExpect<'a, View>,
        seers: WriteStorage<'a, Seeing>,
        message_log: WriteExpect<'a, MessageLog>,
        tile_map: WriteExpect<'a, TileMap>,
        // turn_queue: WriteExpect<'a, crate::TurnQueue>,
}

pub struct Render;
impl<'a> System<'a> for Render {
    type SystemData = RenderData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let viewport = Viewport {
            width: SCREEN_WIDTH - 31,
            height: SCREEN_HEIGHT - 2
        };

        {
            let mut fov_map = data.view.map.lock().unwrap();
            for (ent, pos, _player) in (&data.entities, &data.positions, &data.players).join() {
                fov_map.compute_fov(pos.x, pos.y, 100, true, FovAlgorithm::Basic);
            }
        }

        // tcod::system::set_fps(4);
        if data.world_resources.player_turn {
            viewport.update(&mut data);
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

    }
}