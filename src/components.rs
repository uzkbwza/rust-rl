use specs::prelude::*;
use rltk::RGB;
use crate::systems::ai::types::AiType;
use std::collections::HashMap;
use crate::BASE_TURN_TIME;
use crate::MIN_TURN_TIME;
use crate::systems::render::Elevation;

pub mod flags;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct PrintDebug;

#[derive(Component, PartialEq, Debug)]
#[storage(VecStorage)]
pub struct Target {
    pub entity: Entity,
    pub give_up_timer: u32,
}

impl Target {
    pub fn new(entity: Entity) -> Self {
        Target {
            entity,
            give_up_timer: 15
        }
    }

    pub fn decrement_timer(&mut self) {
        self.give_up_timer = self.give_up_timer - 1
    }
}

#[derive(Component, Debug, PartialEq)]
#[storage(VecStorage)]
pub struct Name {
    pub name: String
}

impl Name {
    pub fn new(name: &str) -> Self {
        Name {
            name: String::from(name)
        }
    }
}

#[derive(Component, PartialEq, Debug)]
#[storage(DenseVecStorage)]
pub struct Quickness {
    pub quickness: u32
}

impl Quickness {
    pub fn new() -> Self {
        Quickness {
            quickness: BASE_TURN_TIME
        }
    }

    pub fn modify_quickness(&mut self, modifier: i32) {
        if modifier < BASE_TURN_TIME as i32 {
            self.quickness = (BASE_TURN_TIME as i32 - modifier) as u32;
        } else {
            self.quickness = MIN_TURN_TIME
        }
    }
}

#[derive(Component, Copy, Clone, PartialEq, Debug)]
#[storage(DenseVecStorage)]
pub struct Actor {
    pub next_turn: u64,
    pub stats: Stats
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Stats {
    pub strength: u32,
    pub agility: u32,
    pub intelligence: u32,
}

impl Actor {
    pub fn _new() -> Self {
        let stats = Stats {
                strength: 10,
                agility: 10,
                intelligence: 10 };

        Actor { 
            next_turn: 0,
            stats
        }
    }

    pub fn from_stats(strength: u32, agility: u32, intelligence: u32) -> Self {
        Actor {
            next_turn: 0,
            stats: Stats {
                strength,
                agility,
                intelligence
            }
        }
    }

    pub fn set_next_turn_from_cost(&mut self, world_time: u64, cost: u32) {
        self.next_turn = u64::max(world_time + 1, world_time + cost as u64)
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
// something that is destructible and physically exists
pub struct Corporeal {
    pub max_hp: i32,
    pub hp: i32,
    pub base_damage: i32,
}

impl Corporeal {
    pub fn new(max_hp: i32, base_damage: i32) -> Self {
        Corporeal {
            max_hp,
            hp: max_hp,
            base_damage,
        }
    }
}

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct MyTurn;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct PlayerControl;

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct AiControl {
    pub ai_type: AiType,
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
#[storage(VecStorage)]
pub struct Position{
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Renderable {
    pub glyph: char,
    pub fg_color: (u8, u8, u8),
    pub bg_color: Option<(u8, u8, u8)>,
    pub elevation: Elevation
}

impl Renderable {
    pub fn new(glyph: char, fg_color: (u8, u8, u8), bg_color: Option<(u8, u8, u8)>, elevation: Elevation) -> Self {
        Renderable { glyph, fg_color, bg_color, elevation }
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct BoxRenderable {
    pub map: Vec<Vec<(char, RGB)>>,
}

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct Defending {
    pub damage_source: Entity,
    pub damage_amount: i32,
}

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Floor;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct OnFloor;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Camera;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct InView;

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Seeing {
    pub fov: i32,
    pub seen: HashMap<(i32, i32), char>,
}

impl Seeing {
    pub fn new(fov: i32) -> Self {
        Seeing {
            fov,
            seen: HashMap::new()
        }
    }
}

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct BlockSight;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Invulnerable;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct BlockMovement;

#[derive(Component, PartialEq, Default, Debug)]
#[storage(NullStorage)]
pub struct CanSeeTarget;