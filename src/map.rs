use specs::prelude::*;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct EntityMap {
    pub colliders: HashMap<(i32, i32), Entity>,
    pub renderables: HashMap<(i32, i32), Entity>,
}

impl EntityMap {
    pub fn _new() -> Self {
        let colliders: HashMap<(i32, i32), Entity> = HashMap::new();
        let renderables: HashMap<(i32, i32), Entity> = HashMap::new();
        EntityMap {
            colliders,
            renderables,
        }
    }
}
