use specs::prelude::*;
use std::collections::HashMap;
use tcod::map::Map as TcodMap;
use std::sync::{Arc, Mutex};

#[derive(Default, Debug)]
pub struct EntityMap {
    pub colliders: HashMap<(i32, i32), Entity>,
    pub renderables: HashMap<(i32, i32), Entity>,
}

impl EntityMap {
    pub fn new() -> Self {
        let colliders: HashMap<(i32, i32), Entity> = HashMap::new();
        let renderables: HashMap<(i32, i32), Entity> = HashMap::new();
        EntityMap {
            colliders,
            renderables,
        }
    }
}

// use for pathfinding, fov, etc. essentially just a wrapper for tcod's map
// currently not actually used for anything...
pub struct View {
    pub map: Arc<Mutex<TcodMap>>
}