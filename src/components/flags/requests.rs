use specs::prelude::*;
use tcod::colors;
use crate::systems::ai::types::AiType;
use crate::systems::movement::Dir;
use std::collections::HashMap;

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct MoveRequest {
    pub dx: i32,
    pub dy: i32,
}

impl MoveRequest {
    pub fn new(dx: i32, dy: i32) -> Self {
        MoveRequest {
            dx,
            dy,
        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct AttackRequest {
    pub dir: Dir,
}

impl AttackRequest {
    pub fn new(dir: Dir) -> Self {
        AttackRequest {
            dir
        }
    }
}