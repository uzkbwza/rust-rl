use specs::prelude::*;
use crate::{MAP_WIDTH, MAP_HEIGHT};
use crate::components::*;
use tcod::colors;
use rltk::RGB;
use array2d::Array2D;
pub mod algorithms;
use algorithms::TileType;
use rand::prelude::*;

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
    corporeals: WriteStorage<'a, Corporeal>,
    positions: WriteStorage<'a, Position>,
    renderables: WriteStorage<'a, Renderable>,
    movement_blockers: WriteStorage<'a, BlockMovement>,
    sight_blockers: WriteStorage<'a, BlockSight>,
    floors: WriteStorage<'a, Floor>,
    world_updater: Read<'a, LazyUpdate>,
}

impl<'a> System<'a> for MapGen {
    type SystemData = MapGenSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        if self.initialized { return }
        let map = algorithms::random(MAP_WIDTH as usize, MAP_HEIGHT as usize);
        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                let tile_entity = data.entities.create();
                let tile_type = map[(x as usize, y as usize)];
                data.positions.insert(tile_entity, Position::new(x, y));

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
                            let color = RGB::from_u8(r, r, r );
                            let bg_color =  RGB::from_u8(r - 5, g - 5, b - 5,);

                        data.renderables.insert(tile_entity, Renderable::new(random_char, color, Some(bg_color)));
                        data.floors.insert(tile_entity, Floor{});
                    },

                    TileType::Wall => {
                        data.renderables.insert(tile_entity, Renderable::new('#', RGB::from_u8(255,255,255), Some(RGB::from_u8(20,20,20))));
                        data.corporeals.insert(tile_entity, Corporeal::new(1000));
                        data.movement_blockers.insert(tile_entity, BlockMovement{});
                        data.sight_blockers.insert(tile_entity, BlockSight{});
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

pub struct MapPopulator;