pub mod requests;
use specs::prelude::*;
use crate::BASE_TURN_TIME;

#[derive(Component, Debug)]
pub struct ActionResult {
    pub cost: u32
}

impl ActionResult {
    pub fn from(cost: u32) -> Self {
        ActionResult {
            cost
        }
    }
}