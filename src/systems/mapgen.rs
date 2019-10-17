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
    initialized: bool
}

impl MapGen {
    pub fn new() -> Self {
        MapGen {
            initialized: false
        }
    }
}

#[derive(SystemData)]
pub struct MapGenSystemData<'a> {
    entities: Entities<'a>,
    world_updater: Read<'a, LazyUpdate>,
    entity_load_queue: WriteExpect<'a, EntityLoadQueue>
}

impl<'a> System<'a> for MapGen {
    type SystemData = MapGenSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if self.initialized { return }
        let seed: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect();

        println!{"{}", seed};

        let mut bsp_level = BspLevel::create(CONFIG.map_width, CONFIG.map_height, &seed);
        let mut player_placed = false;
        let mut dummy_placed = false;
        let mut rng = thread_rng();

        for (i, tile) in bsp_level.tile_map.items.iter().enumerate() {
            let (x, y) = bsp_level.tile_map.idx_xy(i);
            let blueprint = EntityBlueprint::load_and_place("terrain/base_floor".to_string(), x, y);
            data.entity_load_queue.push(blueprint);

            match *tile {
                TileType::Wall => {
                    let blueprint = EntityBlueprint::load_and_place("terrain/base_wall".to_string(), x, y);
                    data.entity_load_queue.push(blueprint);
                },
                _ => (),
            }

            if !player_placed && rng.gen_bool(0.005) {
                match *tile {
                    TileType::Floor => {
                        let mut player = EntityBlueprint::load_and_place("player".to_string(), x, y);
                        data.entity_load_queue.push(player);
                        player_placed = true;
                    },
                    _ => (),

                }
            }
        }

        self.initialized = true;
    }
}