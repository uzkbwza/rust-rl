use specs::prelude::*;
use crate::prelude::*;

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Name {
    name: String
}

// #[derive(Component, Default, Debug)]
// #[storage(NullStorage)]
// pub struct Actor;

// #[derive(Component, Debug)]
// #[storage(DenseVecStorage)]
// pub struct Action {
//     pub command: Command
// }

// impl Action {
//     pub fn new(command: Command) -> Self {
//         Action {command}
//     }
// }

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct PlayerControl;

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct AiControl {
    ai_type: AiType,
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
    pub glyph: char
}

impl Renderable {
    pub fn new(glyph: char) -> Self {
        Renderable { glyph }
    }
}