pub mod requests;
use specs::prelude::*;
use crate::BASE_TURN_TIME;

#[derive(Component, Debug)]
pub struct ActionResult {
    pub cost: i32
}

impl ActionResult {
    pub fn from(cost: i32) -> Self {
        ActionResult {
            cost
        }
    }
}