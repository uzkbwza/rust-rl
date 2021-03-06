use crate::bodyparts::*;
use crate::command::Command;
use crate::systems::ai::types::AiType;
use crate::CONFIG;
use serde::Deserialize;
use specs::prelude::*;
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
    pub position: Position,
}

impl Target {
    pub fn new(entity: Entity, position: Position) -> Self {
        Target {
            entity,
            position,
            give_up_timer: 15,
        }
    }

    pub fn decrement_timer(&mut self) {
        self.give_up_timer = self.give_up_timer - 1
    }
}

#[derive(Component, Debug, PartialEq, Deserialize, Clone)]
#[storage(VecStorage)]
pub struct Name {
    #[serde(default)]
    pub name: String,
}

impl Name {
    pub fn new(name: &str) -> Self {
        Name {
            name: String::from(name),
        }
    }
}

#[derive(Component, PartialEq, Clone, Deserialize, Debug)]
#[storage(DenseVecStorage)]
pub struct Mobile {
    #[serde(default)]
    pub quickness: u32,
}

impl Default for Mobile {
    fn default() -> Self {
        Mobile {
            quickness: CONFIG.base_turn_time,
        }
    }
}

impl Mobile {
    pub fn modify_quickness(&mut self, modifier: i32) {
        if modifier < CONFIG.base_turn_time as i32 {
            self.quickness = (CONFIG.base_turn_time as i32 - modifier) as u32;
        } else {
            self.quickness = CONFIG.min_turn_time
        }
    }
}

#[derive(Component, Clone, PartialEq, Debug)]
pub struct CommandSequence {
    pub commands: Vec<Command>,
}

impl Default for CommandSequence {
    fn default() -> Self {
        CommandSequence {
            commands: Vec::new(),
        }
    }
}

#[derive(Component, Clone, PartialEq, Debug, Deserialize)]
#[storage(DenseVecStorage)]
pub struct Actor {
    #[serde(default)]
    pub next_turn: u64,

    #[serde(default)]
    pub stats: Stats,
}

#[derive(PartialEq, Copy, Clone, Debug, Deserialize)]
pub struct Stats {
    pub strength: u32,
    pub agility: u32,
    pub intelligence: u32,
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            strength: 10,
            agility: 10,
            intelligence: 10,
        }
    }
}

impl Actor {
    pub fn new() -> Self {
        let stats = Stats {
            strength: 0,
            agility: 0,
            intelligence: 0,
        };

        Actor {
            next_turn: 0,
            stats,
        }
    }

    pub fn from_stats(strength: u32, agility: u32, intelligence: u32) -> Self {
        Actor {
            next_turn: 0,
            stats: Stats {
                strength,
                agility,
                intelligence,
            },
        }
    }

    pub fn set_next_turn_from_cost(&mut self, world_time: u64, cost: u32) {
        self.next_turn = u64::max(world_time + 1, world_time + cost as u64)
    }
}

impl Default for Actor {
    fn default() -> Self {
        Actor {
            next_turn: 0,
            stats: Stats::default(),
        }
    }
}

#[derive(Component, Clone, Deserialize, Debug)]
#[storage(VecStorage)]
// something that is destructible and physically exists
pub struct Corporeal {
    pub max_hp: i32,
    pub hp: i32,
    pub base_damage: i32,
}

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct MyTurn;

#[derive(Component, Default, Deserialize, Clone, Debug)]
#[storage(NullStorage)]
pub struct PlayerControl;

#[derive(Component, Debug, Clone, Deserialize)]
#[storage(DenseVecStorage)]
pub struct AiControl {
    pub ai_type: AiType,
}

#[derive(Component, Clone, Deserialize, Copy, Debug, PartialEq)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

// TODO: separate elevation into its own component type

#[derive(Component, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Deserialize)]
#[storage(VecStorage)]
pub enum Elevation {
    Floor,
    OnFloor,
    Upright,
    _InAir,
}

#[derive(Component, Clone, Deserialize, Debug)]
#[storage(VecStorage)]
pub struct Renderable {
    pub glyph: char,
    pub fg_color: (u8, u8, u8),
    pub bg_color: Option<(u8, u8, u8)>,
}

impl Renderable {
    pub fn new(glyph: char, fg_color: (u8, u8, u8), bg_color: Option<(u8, u8, u8)>) -> Self {
        Renderable {
            glyph,
            fg_color,
            bg_color,
        }
    }
}

#[derive(Component, Clone, Deserialize, Debug)]
#[storage(VecStorage)]
pub struct RandomRenderable {
    pub glyphs: String,
    pub fg_colors: Vec<(u8, u8, u8)>,
    pub bg_colors: Option<Vec<(u8, u8, u8)>>,
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

#[derive(Component, Default, Deserialize, Clone, Debug)]
#[storage(NullStorage)]
pub struct Camera;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct InView;

#[derive(Component, Clone, Debug, Deserialize)]
#[storage(VecStorage)]
pub struct Seeing {
    #[serde(default)]
    pub fov: i32,

    #[serde(default)]
    pub seen: HashMap<(i32, i32), char>,
}

impl Default for Seeing {
    fn default() -> Self {
        Seeing {
            fov: 10,
            seen: HashMap::new(),
        }
    }
}

impl Seeing {
    pub fn new(fov: i32) -> Self {
        Seeing {
            fov,
            seen: HashMap::new(),
        }
    }
}

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Death;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Corpse;

#[derive(Component, Default, Debug, Clone, Deserialize)]
#[storage(NullStorage)]
pub struct BlockSight;

#[derive(Component, Default, Debug, Clone, Deserialize)]
#[storage(NullStorage)]
pub struct Invulnerable;

#[derive(Component, Default, Debug, Clone, Deserialize)]
#[storage(NullStorage)]
pub struct BlockMovement;

#[derive(Component, PartialEq, Default, Debug)]
#[storage(NullStorage)]
pub struct CanSeeTarget;

#[derive(Component, PartialEq, Default, Debug)]
#[storage(NullStorage)]
pub struct Carryable;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Inventory {
    pub items: Vec<Entity>,
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Body {
    core: BodyPart,
}

impl Body {
    pub fn new() -> Self {
        let core = BodyPart {
            name: String::from("Core"),
            children: Vec::new(),
            tags: vec![BodyPartTag::Core],
            armor_tags: vec![ArmorTag::Core],
            equipped_armor: Vec::new(),
        };

        Body { core }
    }

    // TODO: use something like JSON to store bodypart templates
    pub fn make_humanoid() -> Self {
        let mut core = BodyPart {
            name: String::from("Core"),
            children: Vec::new(),
            tags: vec![BodyPartTag::Core],
            armor_tags: vec![ArmorTag::Core],
            equipped_armor: Vec::new(),
        };

        let mut head = BodyPart {
            name: String::from("Head"),
            children: Vec::new(),
            tags: vec![BodyPartTag::ThoughtCenter, BodyPartTag::Limb],
            armor_tags: vec![ArmorTag::Head, ArmorTag::Jewelry(3)],
            equipped_armor: Vec::new(),
        };

        let mut left_arm = BodyPart {
            name: String::from("Left Arm"),
            children: Vec::new(),
            tags: vec![BodyPartTag::Limb],
            armor_tags: vec![ArmorTag::Arm],
            equipped_armor: Vec::new(),
        };

        let mut right_arm = BodyPart {
            name: String::from("Right Arm"),
            children: Vec::new(),
            tags: vec![BodyPartTag::Limb],
            armor_tags: vec![ArmorTag::Arm],
            equipped_armor: Vec::new(),
        };

        let mut left_hand = BodyPart {
            name: String::from("Left Hand"),
            children: Vec::new(),
            //MY GAME MY RULES
            tags: vec![
                BodyPartTag::Limb,
                BodyPartTag::Grasping,
                BodyPartTag::Dominant,
            ],
            armor_tags: vec![ArmorTag::Hand, ArmorTag::Jewelry(5)],
            equipped_armor: Vec::new(),
        };

        let mut right_hand = BodyPart {
            name: String::from("Right Hand"),
            children: Vec::new(),
            tags: vec![BodyPartTag::Limb, BodyPartTag::Grasping],
            armor_tags: vec![ArmorTag::Head, ArmorTag::Jewelry(5)],
            equipped_armor: Vec::new(),
        };

        let mut left_leg = BodyPart {
            name: String::from("Left Leg"),
            children: Vec::new(),
            tags: vec![BodyPartTag::Limb, BodyPartTag::Mobility],
            armor_tags: vec![ArmorTag::Head],
            equipped_armor: Vec::new(),
        };

        let mut right_leg = BodyPart {
            name: String::from("Right Leg"),
            children: Vec::new(),
            tags: vec![BodyPartTag::Limb, BodyPartTag::Mobility],
            armor_tags: vec![ArmorTag::Head],
            equipped_armor: Vec::new(),
        };

        let mut left_foot = BodyPart {
            name: String::from("Left Foot"),
            children: Vec::new(),
            tags: vec![BodyPartTag::Limb],
            armor_tags: vec![ArmorTag::Head],
            equipped_armor: Vec::new(),
        };

        let mut right_foot = BodyPart {
            name: String::from("Right Foot"),
            children: Vec::new(),
            tags: vec![BodyPartTag::Limb],
            armor_tags: vec![ArmorTag::Head],
            equipped_armor: Vec::new(),
        };

        left_arm.add_child(left_hand);
        left_leg.add_child(left_foot);
        right_arm.add_child(right_hand);
        right_leg.add_child(right_foot);

        core.add_child(head);
        core.add_child(left_arm);
        core.add_child(left_leg);
        core.add_child(right_arm);
        core.add_child(right_leg);

        Body { core }
    }
}
