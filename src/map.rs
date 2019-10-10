use specs::prelude::*;
use tcod::map::Map as TcodMap;
use std::sync::{Arc, Mutex};
use crate::CONFIG;
use vecmap::*;

type ActorMap = VecMap<Option<Entity>>;

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
    pub map: Arc<Mutex<TcodMap>>
}