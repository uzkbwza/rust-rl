use specs::prelude::*;
use crate::{MAP_WIDTH, MAP_HEIGHT};
use crate::components::*;
pub mod algorithms;
use algorithms::TileType;
use rand::prelude::*;

pub struct MapGen {
    initialized: bool
}

impl MapGen {
    pub fn _new() -> Self {
        MapGen {
            initialized: false
        }
    }
}

#[derive(SystemData)]
pub struct MapGenSystemData<'a> {
    entities: Entities<'a>,
    world_updater: Read<'a, LazyUpdate>,
}

impl<'a> System<'a> for MapGen {
    type SystemData = MapGenSystemData<'a>;
    fn run(&mut self, data: Self::SystemData) {
        if self.initialized { return }
        let map = algorithms::random(MAP_WIDTH as usize, MAP_HEIGHT as usize);
        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                let tile_entity = data.entities.create();
                let tile_type = map[(x as usize, y as usize)];
                data.world_updater.insert(tile_entity, Position::new(x, y));

                match tile_type {
                    TileType::Floor => {
                        let mut rng = rand::thread_rng();
                            let brightness: i16 = 20;
                            let variation: i16 = 5;
                            let chars = "rn.,` ";
                            let random_char = chars
                                .chars()
                                .choose(&mut rng)
                                .unwrap();
                            let r = (5 + rng.gen_range(0, variation)) as u8;
                            let g = (brightness + rng.gen_range(-variation, variation)) as u8;
                            let b = (5 + rng.gen_range(0, variation)) as u8;
                            let color = (r, r, r );
                            let bg_color =  (r - 5, g - 5, b - 5,);

                        data.world_updater.insert(tile_entity, Renderable::new(random_char, color, Some(bg_color)));
                        data.world_updater.insert(tile_entity, Floor{});
                    },

                    TileType::Wall => {
                        data.world_updater.insert(tile_entity, Renderable::new('#', (255,255,255), Some((20,20,20))));
                        data.world_updater.insert(tile_entity, Corporeal::new(1000));
                        data.world_updater.insert(tile_entity, BlockMovement{});
                        data.world_updater.insert(tile_entity, BlockSight{});
                    },
                    _ => panic!(format!("{}, {:?}", "unimplemented tile type: ", tile_type)),
                }
            }
        }
        self.initialized = true
    }
    fn setup(&mut self, world: &mut World ) {
        Self::SystemData::setup(world);
    }
}

pub struct _MapPopulator;