use specs::prelude::*;

#[derive(Debug)]
pub enum BodyPartTag {
    // for the main body that everything attaches to
    Core,

    // things that extend off the body and can be dismembered
    Limb,

    // can wield items
    Grasping,

    // used for primary weapon hand, TODO: implement dominant feet/eyes/etc down the line.
    Dominant,

    // contributes to mobility. the more of these a creature has, the smaller the mobility deficit
    // if the limb is dismembered.
    Mobility,

    // if this part is destroyed, the creature dies instantly
    ThoughtCenter,
}

#[derive(Debug)]
// Tells the system what kind of armor can be worn on this body part
pub enum ArmorTag {
    Core,
    Back, // Use with Core
    Head,
    Arm,
    Leg,
    Hand,
    Foot,
    Tail,
    GenericLimb,
    Jewelry(i32), // Amount of jewelry that can be worn on this body part, i.e. hands get 5 each
}

#[derive(Debug)]
pub struct BodyPart {
    pub name: String,
    pub children: Vec<BodyPart>,
    pub tags: Vec<BodyPartTag>,
    pub armor_tags: Vec<ArmorTag>,
    pub equipped_armor: Vec<Entity>,
}

impl BodyPart {
    pub fn add_child(&mut self, body_part: BodyPart) {
        self.children.push(body_part);
    }
}
