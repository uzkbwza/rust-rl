use specs::prelude::*;
use crate::components::*;
use crate::map::{View};
use crate::{MAP_WIDTH, MAP_HEIGHT, VIEWPORT_WIDTH, VIEWPORT_HEIGHT, VIEWPORT_POS_X, VIEWPORT_POS_Y, SCREEN_HEIGHT};
use vecmap::*;
use tcod::map::FovAlgorithm;
use tcod::console::*;
use crate::MessageLog;

pub type TileMap = VecMap<Tile>;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum Elevation {
    Floor,
    OnFloor,
    Upright,
    _InAir,
}

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    pub position: Position,
    pub elevation: Elevation,
    pub glyph: char,
    pub fg_color: (u8, u8, u8),
    pub bg_color: Option<(u8, u8, u8)>,
}

impl Tile {
    pub fn new() -> Self {
        Tile {
            position: Position::new(-1, -1),
            elevation: Elevation::Floor,
            glyph: ' ',
            fg_color: (255, 255, 255),
            bg_color: Some((0, 0, 0))
        }
    }
}

struct Viewport {
    width: i32,
    height: i32,
    seen: TileMap,
}

impl Viewport {

    pub fn set_tile(&self, mut tile: Tile, tile_map: &mut TileMap) {
        let rend_pos = tile.position;
        let (x, y) = (rend_pos.x, rend_pos.y);

        if x < 0 || x >= self.width { return };
        if y < 0 || y >= self.height { return };

        if let Ok(existing_tile) = tile_map.retrieve(x, y) {

            if tile.elevation < existing_tile.elevation {
                tile.glyph = existing_tile.glyph;
                tile.fg_color = existing_tile.fg_color;
            }

            if let Some(_) = existing_tile.bg_color {
                if tile.bg_color == None {
                    tile.bg_color = existing_tile.bg_color
                }
            }

            else {
                if tile.bg_color == None {
                    tile.bg_color = Some((0, 0, 0))
                }
            }


        }

        match tile_map.set_point(x, y, tile) {
            Ok(_) => (),
            Err(e) => println!("{}", e)
        }
    }

    // creates full character map of what the player sees.
    fn set_map(&mut self, data: &mut RenderSystemData) {

        data.tile_map.reset_map();
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

            let screen_pos = self.get_screen_coordinates(*pos, camera_pos);

            let mut tile = Tile {
                position: screen_pos,
                elevation,
                glyph,
                fg_color,
                bg_color,
            };

            let fov_map = data.view.map.lock().unwrap();

            if !fov_map.is_in_fov(pos.x, pos.y) {
                tile = Tile::new();
                if let Ok(t) = self.seen.retrieve(pos.x, pos.y) {
                    tile = t;
                    tile.position = screen_pos;
                    tile.bg_color = None;
                    tile.fg_color = (20, 20, 30);
                } else { return }
            } else {
                self.seen.set_point(pos.x, pos.y, tile);
            }
//
//            println!("{:?}", tile);

            self.set_tile(tile, &mut data.tile_map);
        }
    }

    fn get_screen_coordinates(&self, pos: Position, camera_pos: Position) -> Position {
        let screen_center = Position::new(self.width / 2, self.height / 2);
        Position::new(screen_center.x + pos.x - camera_pos.x, screen_center.y + pos.y - camera_pos.y)
    }

    fn _is_on_screen(&self, coords: Position) -> bool {
        if coords.x >= 0 && coords.x <= self.width && coords.y >= 0 && coords.y <= self.height {
            return true
        }
        false
    }

    fn _get_world_coordinates(&self, rend_pos: Position, camera_pos: Position) -> Position {
        let screen_center = Position::new(self.width / 2, self.height / 2);
        let mut wx = rend_pos.x - screen_center.x + camera_pos.x;
        let mut wy = rend_pos.x - screen_center.x + camera_pos.y;
        if wx > MAP_WIDTH { wx = MAP_WIDTH }
        if wy > MAP_HEIGHT { wy = MAP_HEIGHT }
        Position::new(wx, wy)
    }

    fn get_camera_position(&self,  data: &RenderSystemData) -> Position {
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
pub struct RenderSystemData<'a> {
        entities: Entities<'a>,
        renderables: ReadStorage<'a, Renderable>,
        positions: ReadStorage<'a, Position>,
        players: ReadStorage<'a, PlayerControl>,
        cameras: ReadStorage<'a, Camera>,
        floors:    ReadStorage<'a, Floor>,
        on_floors:    ReadStorage<'a, OnFloor>,
        game_state: ReadExpect<'a, crate::GameState>,
        view: ReadExpect<'a, View>,
        tile_map: WriteExpect<'a, TileMap>,
        console: WriteExpect<'a, Root>,
        message_log: WriteExpect<'a, MessageLog>,
        // turn_queue: WriteExpect<'a, crate::TurnQueue>,
}

pub struct RenderViewport {
    viewport: Option<Viewport>
}

impl RenderViewport {
    pub fn new() -> Self {
        let viewport = Some(Viewport {
            width: VIEWPORT_WIDTH, 
            height: VIEWPORT_HEIGHT,
            seen: TileMap::filled_with(Tile::new(), MAP_WIDTH, MAP_HEIGHT)
        });
        
        RenderViewport {
            viewport
        }
    }

    pub fn render(console: &mut Root, tile_map: &TileMap) {
        for tile in tile_map.items.iter() {
            Self::render_char(console, *tile);
        }
    }

    pub fn render_char(console: &mut Root, tile: Tile) {

        if tile.position.x < 0 || tile.position.x >= console.width() { return };
        if tile.position.y < 0 || tile.position.y >= console.height() { return };

//        println!("{:?}", tile);
        let (fg_r, fg_g, fg_b) = tile.fg_color;
        let (bg_r, bg_g, bg_b) = tile.bg_color.expect("Tile does not have background value!");

        let fg_color = tcod::colors::Color{ r: fg_r, g: fg_g, b: fg_b };
        let bg_color = tcod::colors::Color{ r: bg_r, g: bg_g, b: bg_b };

        console.put_char_ex(
            tile.position.x + VIEWPORT_POS_X,
            tile.position.y + VIEWPORT_POS_Y,
            tile.glyph,
            fg_color,
            bg_color
        );

    }
}

impl<'a> System<'a> for RenderViewport {
    type SystemData = RenderSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {

        if !data.game_state.player_turn {
            return
        }

        {
            let mut fov_map = data.view.map.lock().unwrap();
            for (pos, _player) in (&data.positions, &data.players).join() {
                fov_map.compute_fov(pos.x, pos.y, 100, true, FovAlgorithm::Basic);
            }
        }

        let mut viewport = self.viewport.as_mut().unwrap();
        viewport.set_map(&mut data);

        let tile_map = &data.tile_map;
        let console = &mut data.console;

        console.clear();
        Self::render(console, tile_map);
    }
}

pub struct RenderUi;
impl<'a> System<'a> for RenderUi {
    type SystemData = RenderSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        let message_log = data.message_log;
        let console = &mut data.console;
        let message_log_height = (SCREEN_HEIGHT - VIEWPORT_HEIGHT - 1) as usize;
        for (i, message) in message_log.messages.iter().enumerate() {
            if i > message_log_height { break }
            console.print(0,SCREEN_HEIGHT - 1 - i as i32, message);
        }
    }
}