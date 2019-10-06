use specs::prelude::*;
use crate::{MAP_WIDTH, MAP_HEIGHT};
use crate::components::*;
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
