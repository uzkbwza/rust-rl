use specs::prelude::*;
use tcod::map::Map as TcodMap;
use std::sync::{Arc, Mutex};
use crate::{MAP_WIDTH, MAP_HEIGHT};

#[derive(Debug, Clone)]
pub struct VecMap<T> where T: Clone + Copy {
    pub items: Vec<T>,
    default: T
}

impl<T> VecMap<T> where T: Clone + Copy {
    pub fn filled_with(item: T) -> Self {
        let items = vec![item; (MAP_WIDTH * MAP_HEIGHT) as usize];
        let default = item;
        VecMap {
            items,
            default
        }
    }

    pub fn retrieve(&self, x: i32, y: i32) -> Option<T> {
        let id = Self::xy_idx(x, y);
        if id < self.items.len() {
            return Some(self.items[id])
        }
        return None
    }

    pub fn set_point(&mut self, x: i32, y: i32, item: T) {
        self.items[Self::xy_idx(x, y)] = item;
    }

    pub fn reset_point(&mut self, x: i32, y: i32) {
        let id = Self::xy_idx(x, y);
        if id < self.items.len() {
            self.items[id] = self.default;        
        }
    }

    pub fn reset_map(&mut self) {
        for i in 0..self.items.len() {
            let (x, y) = Self::idx_xy(i);
            self.reset_point(x, y);
        }
    }
    // stealing this from thebracket

    // We're storing all the tiles in one big array, so we need a way to map an X,Y coordinate to
    // a tile. Each row is stored sequentially (so 0..80, 81..160, etc.). This takes an x/y and returns
    // the array index.
    pub fn xy_idx(x: i32, y: i32) -> usize {
        (y as usize * MAP_WIDTH as usize) + x as usize
    }

    // It's a great idea to have a reverse mapping for these coordinates. This is as simple as
    // index % MAP_WIDTH (mod MAP_WIDTH), and index / MAP_WIDTH
    pub fn idx_xy(idx: usize) -> (i32, i32) {
        (idx as i32 % MAP_WIDTH, idx as i32 / MAP_WIDTH)
    }
}

type ActorMap = VecMap<Option<Entity>>;

#[derive(Debug)]
pub struct EntityMap {
    pub actors: ActorMap,
    pub width: usize,
    pub height: usize,
}

impl EntityMap {
    pub fn new(width: usize, height: usize) -> Self {
        let actor_map = ActorMap::filled_with(None);
        
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