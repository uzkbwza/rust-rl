use crate::CONFIG;
use specs::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub type TurnQueue = BinaryHeap<Turn>;

pub struct WorldTime {
    pub tick: u64,
    pub world_turns: u32,
    pub player_turns: u32,
}

#[derive(Eq, Debug)]
pub struct Turn {
    pub tick: u64,
    pub entity: Entity,
}

impl Ord for Turn {
    fn cmp(&self, other: &Self) -> Ordering {
        other.tick.cmp(&self.tick)
    }
}

impl PartialOrd for Turn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Turn {
    fn eq(&self, other: &Self) -> bool {
        self.tick == other.tick
    }
}

impl WorldTime {
    pub fn new() -> Self {
        WorldTime {
            tick: 0,
            world_turns: 0,
            player_turns: 0,
        }
    }

    pub fn determine_world_turn(&mut self) {
        self.world_turns = (self.tick / CONFIG.base_turn_time as u64) as u32
    }

    pub fn increment_player_turn(&mut self) {
        self.player_turns += 1;
    }
}
