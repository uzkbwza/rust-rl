use specs::prelude::*;
use crate::components::*;

pub fn create_player(world: &mut World, x: i32, y: i32) {
    world.create_entity()
        .with(PlayerControl{})
        .with(Position::new(x,y))
        .with(Renderable::new('@'))
        .build();
}