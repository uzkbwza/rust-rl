use specs::prelude::*;
use tcod::colors;
use crate::systems::ai::AiType;
use std::collections::HashMap;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct PrintDebug;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Target {
    pub entity: Entity,
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
}

impl Actor {
    pub fn new() -> Self {
        Actor { fatigue: 0.0 }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct CostMultiplier {
    pub multiplier: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
// something that is destructible and physically exists
pub struct Corporeal {
    pub max_hp: i32,
    pub hp: i32
}

impl Corporeal {
    pub fn _new(max_hp: i32) -> Self {
        Corporeal {
            max_hp,
            hp: max_hp,
        }
    }
}

#[derive(Component, Default, Debug, PartialEq)]
#[storage(NullStorage)]
pub struct Collidable;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Stats {
    pub strength: i32,
    pub agility: i32,
    pub intelligence: i32,
}

impl Stats {
    pub fn new(strength: i32, agility: i32, intelligence: i32) -> Self {
        Stats {
            strength,
            agility,
            intelligence,
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

#[derive(Component, Debug)]
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
    pub color: colors::Color,
}

impl Renderable {
    pub fn new(glyph: char, color: colors::Color) -> Self {
        Renderable { glyph, color }
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