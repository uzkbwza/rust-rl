use specs::prelude::*;
use std::collections::HashMap;
use tcod::map::Map as TcodMap;
use std::sync::{Arc, Mutex};
use array2d::Array2D;

#[derive(Debug)]
pub struct ActorMap{
    actors: Array2D<Option<Entity>>
}

impl ActorMap {
    pub fn new(width: usize, height: usize) -> Self {
        let mut actors: Array2D<Option<Entity>> = Array2D::filled_with(None, width, height);
        ActorMap {
            actors
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<Entity> {
        if let Some(actor) = self.actors.get(x as usize, y as usize) {
            return *actor
        }
        return None
    }

    pub fn contains_actor(&self, x: i32, y: i32) -> bool {
        if let Some(entity) = self.get(x, y) {
            return true
        }
        false
    }

    pub fn insert(&mut self, x: i32, y: i32, entity: Entity) {
        self.actors[(x as usize, y as usize)] = Some(entity);
    }

    pub fn remove(&mut self, x: i32, y: i32) {
        self.actors[(x as usize, y as usize)] = None;
    }
}

#[derive(Debug)]
pub struct EntityMap {
    pub actors: ActorMap,
    pub width: usize,
    pub height: usize,
}

impl EntityMap {
    pub fn new(width: usize, height: usize) -> Self {
        let mut actors = ActorMap::new(width, height);
        EntityMap {
            actors,
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