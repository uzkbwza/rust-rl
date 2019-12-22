use crate::components::*;
use crate::entity_factory::EntityLoadQueue;
use crate::mapgen::bsp::*;
use crate::GameState;
use crate::mapgen::level::*;
use crate::mapgen::*;
use crate::CONFIG;
use rand;
use rand::distributions::{Alphanumeric, Standard};
use rand::prelude::*;
use sha2::{Digest, Sha256};
use specs::prelude::*;
use crate::ecs::State;

pub struct MapGen {
    initialized: bool,
}

impl MapGen {
    pub fn new() -> Self {
        MapGen { initialized: false }
    }
}

#[derive(SystemData)]
pub struct MapGenSystemData<'a> {
    entities: Entities<'a>,
    world_updater: Read<'a, LazyUpdate>,
    entity_load_queue: WriteExpect<'a, EntityLoadQueue>,
    game_state: WriteExpect<'a, GameState>,
}

impl<'a> System<'a> for MapGen {
    type SystemData = MapGenSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let seed: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect();

        println! {"Seed: {}", seed};

        let mut bsp_level = BspLevel::create(CONFIG.map_width, CONFIG.map_height, &seed);
//        println!("{}", bsp_level);
        let mut player_placed = false;
        let mut dummy_placed = false;
        let mut rng = thread_rng();

        for (i, tile) in bsp_level.tile_map.items.iter().enumerate() {
            let (x, y) = bsp_level.tile_map.idx_xy(i);
            data.entity_load_queue
                .push(("terrain.base_floor".to_string(), Some(Position::new(x, y))));

            match *tile {
                TileType::Wall => {
                    data.entity_load_queue
                        .push(("terrain.base_wall".to_string(), Some(Position::new(x, y))));
                }
                _ => (),
            }

            if rng.gen_bool(0.05) {
                data.entity_load_queue
                    .push(
                        (
                            "creatures.base_monster".to_string(),
                            Some(Position::new(x, y))
                        )
                    );
            }

            if !player_placed {
                match *tile {
                    TileType::Floor => {
                        data.entity_load_queue
                            .push(("player".to_string(), Some(Position::new(x, y))));
                        player_placed = true;
                    }
                    _ => (),
                }
            }
        }

        data.game_state.transition(State::TurnProcess);
    }
}
