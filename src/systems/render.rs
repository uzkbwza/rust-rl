use specs::prelude::*;
use crate::components::*;
use crate::map::{View, EntityMap};
use vecmap::*;
use tcod::map::FovAlgorithm;
use tcod::console::*;
use tcod::Map as TcodMap;
use crate::MessageLog;
use crate::CONFIG;
use std::sync::MutexGuard;
use serde::Deserialize;
use rand::prelude::*;

pub type TileMap = VecMap<Tile>;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Deserialize)]
pub enum Elevation {
    Floor,
    OnFloor,
    Upright,
    _InAir,
}

pub struct LayeredTileMap {
    pub floor_tiles: TileMap,
    pub on_floor_tiles: TileMap,
    pub upright_tiles: TileMap,
}

impl LayeredTileMap {
    pub fn new(width: i32, height: i32) -> Self {
        LayeredTileMap {
            floor_tiles: TileMap::filled_with(Tile::new(), width, height),
            on_floor_tiles: TileMap::filled_with(Tile::new(), width, height),
            upright_tiles: TileMap::filled_with(Tile::new(), width, height),
        }
    }
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

    pub fn set_tile(&self, mut tile: Tile, layered_tile_map: &mut LayeredTileMap) {
        let rend_pos = tile.position;
        let (x, y) = (rend_pos.x, rend_pos.y);

        if x < 0 || x >= self.width { return };
        if y < 0 || y >= self.height { return };

        let tile_map = match tile.elevation {
            Elevation::Floor => &mut layered_tile_map.floor_tiles,
            Elevation::OnFloor => &mut layered_tile_map.on_floor_tiles,
            Elevation::Upright => &mut layered_tile_map.upright_tiles,
            _ => unimplemented!(),
        };

        match tile_map.set_point(x, y, tile) {
            Ok(_) => (),
            Err(e) => println!("{}", e)
        }
    }

    // creates full character map of what the player sees and has seen.
    fn set_map(&mut self, data: &mut RenderSystemData) {

        let camera_pos = self.get_camera_position(data);
        for (ent, pos, renderable) in (&data.entities, &data.positions, &data.renderables).join() {
            let fov_map = data.view.map.lock().unwrap();
            let (glyph, fg_color, bg_color) = (renderable.glyph, renderable.fg_color, renderable.bg_color);
            let screen_pos = self.get_screen_coordinates(*pos, camera_pos);

            let mut tile = Tile {
                position: screen_pos,
                elevation: renderable.elevation,
                glyph,
                fg_color,
                bg_color,
            };

            // if position isn't in view...
            if !fov_map.is_in_fov(pos.x, pos.y) {

                // reset tile
                tile = Tile::new();

                // check if that position has been seen before,
                // if it has, set our tile to whatever was there, but color it darker.
                if let Ok(t) = self.seen.retrieve(pos.x, pos.y) {
                    tile = t;
                    tile.position = screen_pos;
                    tile.bg_color = Some((10, 10, 10));
                    tile.fg_color = (30, 30, 30);
                } else { return }
            }

            // if we CAN see the position...
            else {
                // if there is already a tile we have seen in that spot...
                if let Ok(seen_tile) = self.seen.retrieve(pos.x, pos.y) {

                    // if that tile should render below or at the same height as whatever is there
                    // now, and what is there now IS NOT an actor (we don't need to save seen
                    // positions of AI's that move around a lot)...
                    if tile.elevation >= seen_tile.elevation && data.actors.get(ent) == None {

                        // change that seen tile to whatever inhabits its position.
                        self.seen.set_point(pos.x, pos.y, tile);
                    }
                } else {
                    self.seen.set_point(pos.x, pos.y, tile);
                }
            }

            if CONFIG.debug_vision {
                tile = self.debug_process_tile(tile, &data, *pos, screen_pos, ent, fov_map)
            }

            self.set_tile(tile, &mut data.layered_tile_map);
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
        if wx > CONFIG.map_width { wx = CONFIG.map_width }
        if wy > CONFIG.map_height { wy = CONFIG.map_height }
        Position::new(wx, wy)
    }

    fn get_camera_position(&self,  data: &RenderSystemData) -> Position {
        let mut camera_position = Position::new(0, 0);
        let viewport_width = self.width;
        let viewport_height = self.height;
        if viewport_width < CONFIG.map_width {
            for (pos, _camera) in (&data.positions, &data.cameras).join() {
                camera_position.x = pos.x;
                if camera_position.x - viewport_width / 2 < 0 {
                    camera_position.x = viewport_width / 2;
                } else if camera_position.x + viewport_width / 2 > CONFIG.map_width {
                    camera_position.x = CONFIG.map_width - viewport_width / 2
                }
            }
        } else {
            camera_position.x = viewport_width / 2
        }

        if viewport_height < CONFIG.map_height {
            for (pos, _camera) in (&data.positions, &data.cameras).join() {
                camera_position.y = pos.y;
                if camera_position.y - viewport_height / 2 < 1 {
                    camera_position.y = viewport_height / 2;
                } else if camera_position.y + viewport_height / 2 > CONFIG.map_height {
                    camera_position.y = CONFIG.map_height - viewport_height / 2
                }
            }
        } else {
            camera_position.y = viewport_height / 2
        }
        camera_position
    }

    fn debug_process_tile(&self, mut tile: Tile, data: &RenderSystemData, pos: Position, screen_pos: Position, ent: Entity, fov_map: MutexGuard<TcodMap>) -> Tile {
        let entity_map = &data.entity_map.actors;

        if !fov_map.is_walkable(pos.x, pos.y) {
            tile.bg_color = Some((255, 0, 0))
        }

        if let Ok(point) = entity_map.retrieve(pos.x, pos.y) {
            if let Some(actor) = point {
                let mut red = 0;
                let mut green = 0;
                if let Some(bg) = tile.bg_color {
                    red = bg.0;
                    green = bg.1;
                }
                tile.bg_color = Some((red, green, 255));
            }
        }

        if let Some(my_turn) = data.my_turns.get(ent) {
            let mut red = 0;
            let mut blue = 0;
            if let Some(bg) = tile.bg_color {
                red = bg.0;
                blue = bg.2;
            }
            tile.bg_color = Some((red, 255, blue));
        }
        tile
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
    layered_tile_map: WriteExpect<'a, LayeredTileMap>,
    console: WriteExpect<'a, Root>,
    message_log: WriteExpect<'a, MessageLog>,
    entity_map: ReadExpect<'a, EntityMap>,
    my_turns: ReadStorage<'a, MyTurn>,
    names: ReadStorage<'a, Name>,
    actors: ReadStorage<'a, Actor>,
}

pub struct RenderViewport {
    viewport: Option<Viewport>
}

impl RenderViewport {
    pub fn new() -> Self {
        let viewport = Some(Viewport {
            width: CONFIG.viewport_width,
            height: CONFIG.viewport_height,
            seen: TileMap::filled_with(Tile::new(), CONFIG.map_width, CONFIG.map_height)
        });
        
        RenderViewport {
            viewport
        }
    }

    pub fn render(console: &mut Root, tile_map: &mut TileMap) {
        for tile in tile_map.items.iter() {
            Self::render_char(console, *tile);
        }
    }

    pub fn render_char(console: &mut Root, tile: Tile) {

        if tile.position.x < 0 || tile.position.x >= console.width() { return };
        if tile.position.y < 0 || tile.position.y >= console.height() { return };

//        println!("{:?}", tile);
        let mut bg_color = console.get_char_background(tile.position.x, tile.position.y);

        let (fg_r, fg_g, fg_b) = tile.fg_color;
        if let Some((r, g, b)) = tile.bg_color {
            bg_color.r = r;
            bg_color.g = g;
            bg_color.b = b;
        }

        let fg_color = tcod::colors::Color{ r: fg_r, g: fg_g, b: fg_b };

        console.put_char_ex(
            tile.position.x + CONFIG.viewport_x,
            tile.position.y + CONFIG.viewport_y,
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
            tcod::system::set_fps(0);
            return
        }

        {
            let mut layered_tile_map = &mut data.layered_tile_map;
            layered_tile_map.floor_tiles.reset_map();
            layered_tile_map.on_floor_tiles.reset_map();
            layered_tile_map.upright_tiles.reset_map();
        }

        tcod::system::set_fps(60);

        {
            let mut fov_map = data.view.map.lock().unwrap();
            for (pos, camera) in (&data.positions, &data.cameras).join() {
                fov_map.compute_fov(pos.x, pos.y, 100, true, FovAlgorithm::Restrictive);
            }
        }

        {
            let mut viewport = self.viewport.as_mut().unwrap();
            viewport.set_map(&mut data);
        }


        let console = &mut data.console;

        console.clear();
        let mut layered_tile_map = &mut data.layered_tile_map;
        Self::render(console, &mut layered_tile_map.floor_tiles);
        Self::render(console, &mut layered_tile_map.on_floor_tiles);
        Self::render(console, &mut layered_tile_map.upright_tiles);
    }
}

pub struct RenderUi;
impl<'a> System<'a> for RenderUi {
    type SystemData = RenderSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        let message_log = data.message_log;
        let console = &mut data.console;
        let message_log_height = (CONFIG.screen_height - CONFIG.viewport_height) as usize;
        let mut formatted_message = String::new();
        for (i, message) in message_log.messages.iter().enumerate() {
            if i > message_log_height { break }
            formatted_message = format!("{}\n{}", message, formatted_message);
        }
        console.print_rect(0,CONFIG.viewport_height, CONFIG.screen_width, message_log_height as i32, formatted_message);
    }
}

#[derive(SystemData)]
pub struct RandomRenderSystemData<'a> {
    entities: Entities<'a>,
    renderables: WriteStorage<'a, Renderable>,
    random_renderables: WriteStorage<'a, RandomRenderable>,
    world_updater: Read<'a, LazyUpdate>,
}

pub struct RandomRender;
impl<'a> System<'a> for RandomRender {
    type SystemData = RandomRenderSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        let mut rng = rand::thread_rng();
        for (random_renderable, ent) in (&mut data.random_renderables, &data.entities).join() {
            let glyph = random_renderable.glyphs
                .chars()
                .choose(&mut rng)
                .unwrap();

            let fg_color = random_renderable.fg_colors.choose(&mut rng).unwrap().clone();
            let mut bg_color = None;
            if let Some(colors) = &random_renderable.bg_colors {
                bg_color = Some(colors.choose(&mut rng).unwrap().clone());
            }

            if let Some(renderable) = data.renderables.get_mut(ent) {
                renderable.glyph = glyph;
                renderable.fg_color = fg_color;
                renderable.bg_color = bg_color;
            } else {
                let new_renderable = Renderable {
                    glyph,
                    fg_color,
                    bg_color,
                    elevation: random_renderable.elevation,
                };
                data.renderables.insert(ent, new_renderable);
            }
            data.world_updater.remove::<RandomRenderable>(ent);
        }
    }
}