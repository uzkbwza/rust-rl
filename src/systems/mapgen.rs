use specs::prelude::*;
use crate::mapgen::*;
use sha2::{ Sha256, Digest };
use rand::prelude::*;
use rand;
use crate::CONFIG;
use crate::mapgen::bsp::*;
use crate::mapgen::level::*;
use crate::components::*;
use crate::entity_factory::{EntityLoadQueue, EntityBlueprint};
use rand::distributions::{Standard, Alphanumeric};

pub struct MapGen {
    initialized: bool,
    player_placed: bool,
    level: Option<Level>
}

impl MapGen {
    pub fn new() -> Self {
        MapGen {
            initialized: false,
            player_placed: false,
            level: None
        }
    }
}

#[derive(SystemData)]
pub struct MapGenSystemData<'a> {
    entities: Entities<'a>,
    world_updater: Read<'a, LazyUpdate>,
    entity_load_queue: WriteExpect<'a, EntityLoadQueue>,
    players: WriteStorage<'a, PlayerControl>,
    positions: WriteStorage<'a, Position>
}

impl<'a> System<'a> for MapGen {
    type SystemData = MapGenSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let mut rng = thread_rng();
        let seed: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect();

        match self.level {
            None => {
                let mut bsp_level = BspLevel::create(CONFIG.map_width, CONFIG.map_height, &seed);
                self.level = Some(bsp_level);
            },
            _ => (),
        }

        let level =  self.level.as_ref().unwrap();

        if !self.initialized {
            let mut dummy_placed = false;
            for (i, tile) in level.tile_map.items.iter().enumerate() {
                let (x, y) = level.tile_map.idx_xy(i);
                let blueprint = EntityBlueprint::load_and_place("terrain/base_floor".to_string(), x, y);
                data.entity_load_queue.push(blueprint);

                match *tile {
                    TileType::Wall => {
                        let blueprint = EntityBlueprint::load_and_place("terrain/base_wall".to_string(), x, y);
                        data.entity_load_queue.push(blueprint);
                    },
                    TileType::Floor => {
                        if rng.gen_bool(0.02) {
                            let blueprint = EntityBlueprint::load_and_place("creatures/base_creature".to_string(), x, y);
                            data.entity_load_queue.push(blueprint);
                        }
                    },
                    _ => (),
                }
            }
            self.initialized = true;
        }

        for (player, pos) in (&data.players, &mut data.positions).join() {
            if !self.player_placed && rng.gen_bool(0.5) {
                for (i, tile) in level.tile_map.items.iter().enumerate() {
                    let (x, y) = level.tile_map.idx_xy(i);
                    match *tile {
                        TileType::Floor => {
                            pos.x = x;
                            pos.y = y;
                            self.player_placed = true
                        },
                        _ => (),
                    }
                }

            }
        }
    }
}