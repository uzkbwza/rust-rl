use specs::prelude::*;
use tcod::colors;
use crate::systems::ai::types::AiType;
use std::collections::HashMap;
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

#[derive(Component, Debug)]
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
pub struct Actor {
    pub fatigue: f32,
    pub stats: Stats
}

#[derive(PartialEq, Debug)]
pub struct Stats {
    pub strength: i32,
    pub agility: i32, 
    pub intelligence: i32,
}

impl Actor {
    pub fn new() -> Self {
        Actor { 
            fatigue: 0.0,
            stats: Stats {
                strength: 10,
                agility: 10,
                intelligence: 10
            }
        }
    }

    pub fn from_stats(strength: i32, agility: i32, intelligence: i32) -> Self {
        Actor {
            fatigue: 0.0,
            stats: Stats {
                strength,
                agility,
                intelligence
            }
        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct CostMultiplier {
    pub multiplier: f32,
}

impl CostMultiplier {
    pub fn reset(&mut self) {
        self.multiplier = 1.0;
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
// something that is destructible and physically exists
pub struct Corporeal {
    pub max_hp: i32,
    pub hp: i32,
}

impl Corporeal {
    pub fn new(max_hp: i32) -> Self {
        Corporeal {
            max_hp,
            hp: max_hp,
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
    pub fg_color: colors::Color,
    pub bg_color: Option<colors::Color>,
}

impl Renderable {
    pub fn new(glyph: char, fg_color: colors::Color, bg_color: Option<colors::Color>) -> Self {
        Renderable { glyph, fg_color, bg_color }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct BoxRenderable {
    pub map: Vec<Vec<(char, colors::Color)>>,
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
pub struct BlockMovement;

#[derive(Component, PartialEq, Default, Debug)]
#[storage(NullStorage)]
pub struct CanSeeTarget;