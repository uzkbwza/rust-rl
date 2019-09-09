use specs::prelude::*;
use crate::components::*;
use crate::systems::control::AiType;
use rand::prelude::*;
use rand::seq::SliceRandom;
use tcod::colors;

pub fn create_player(world: &mut World, x: i32, y: i32) {
    world.create_entity()
        .with(Position::new(x,y))
        .with(Renderable::new('@', colors::WHITE))
        .with(Collidable{})
        // .with(Corporeal::new(10))
        .with(Actor::new())
        .with(PlayerControl{})
        .with(Stats::new(10,10,10))
        .build();
}

pub fn create_dummy(world: &mut World) {
    let mut rng = rand::thread_rng();
    let stats: (i32, i32, i32) = (rng.gen_range(1, 20), rng.gen_range(1, 20), rng.gen_range(1, 20));
    let x: i32 = rng.gen_range(0, crate::SCREEN_WIDTH);
    let y: i32 = rng.gen_range(0, crate::SCREEN_HEIGHT);
    let possible_colors = [
        colors::RED,
        colors::BLUE,
        colors::YELLOW,
        colors::GREEN,
        colors::GOLD,
        colors::PURPLE,
        colors::AZURE,
        colors::AMBER,
        colors::CHARTREUSE,
        colors::CRIMSON,
        colors::FUCHSIA,
        colors::LIME,
        colors::PINK,
        colors::CYAN,  
    ];
    
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()_+=-\\][{}|:;/?><";
    let random_char = chars
        .chars()
        .choose(&mut rng)
        .unwrap();

    let color = *possible_colors.choose(&mut rng).unwrap();
    world.create_entity()
        .with(Position::new(x,y))
        .with(Renderable::new(random_char, color))
        // .with(Corporeal::new(10))
        .with(Collidable{})
        .with(Actor::new())
        .with(AiControl { ai_type: AiType::Dummy })
        .with(Stats::new(stats.0, stats.1, stats.2))
        .build();
}