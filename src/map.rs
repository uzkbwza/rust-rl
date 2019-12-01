use crate::CONFIG;
use specs::prelude::*;
use std::sync::{Arc, Mutex};
use tcod::map::Map as TcodMap;
use vecmap::*;

pub type ActorMap = VecMap<Option<Entity>>;

#[derive(Debug)]
pub struct EntityMap {
    pub actors: ActorMap,
    pub width: usize,
    pub height: usize,
}

impl EntityMap {
    pub fn new(width: usize, height: usize) -> Self {
        let actor_map = ActorMap::filled_with(None, CONFIG.map_width, CONFIG.map_height);

        EntityMap {
            actors: actor_map,
            width,
            height,
        }
    }
}

// use for pathfinding, fov, etc. essentially just a wrapper for tcod's map
// currently not actually used for anything...
pub struct View {
    pub map: Arc<Mutex<TcodMap>>,
    pub block_map: VecMap<BlockTile>,
}

#[derive(Clone, Copy)]
pub struct BlockTile {
    pub blocks_movement: bool,
    pub blocks_sight: bool,
}

impl Default for BlockTile {
    fn default() -> Self {
        BlockTile {
            blocks_movement: false,
            blocks_sight: false,
        }
    }
}
