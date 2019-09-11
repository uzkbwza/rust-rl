use specs::prelude::*;
use crate::components::*;
use crate::systems::ai::AiType;
use rand::prelude::*;
use rand::seq::SliceRandom;
use tcod::colors;

pub fn create_player(world: &mut World, x: i32, y: i32) -> Entity {
    world.create_entity()
        .with(Position::new(x,y))
        .with(Renderable::new('@', colors::WHITE))
        .with(Collidable{})
        // .with(Corporeal::new(10))
        .with(Actor::new())
        .with(PlayerControl{})
        .with(Stats::new(10,20,10))
        .build()
}

pub fn create_dummy(world: &mut World, entity: Entity) {
    let mut rng = rand::thread_rng();
    let stats: (i32, i32, i32) = (rng.gen_range(1, 15), rng.gen_range(1, 19), rng.gen_range(1, 15));
    let x: i32 = rng.gen_range(0, crate::SCREEN_WIDTH);
    let y: i32 = rng.gen_range(0, crate::SCREEN_HEIGHT);
    let possible_colors = [
        colors::GREEN,
        colors::DARK_GREEN,
        colors::DARKER_GREEN,
        colors::LIGHT_GREEN,
        colors::LIGHTER_GREEN,
        colors::LIGHTEST_GREEN,
    ];
    
    let chars = "Zz";
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
        .with(Target { entity })
        .with(AiControl { ai_type: AiType::Dummy })
        .with(Stats::new(stats.0, stats.1, stats.2))
        .build();
}

pub fn create_floor(world: &mut World, x: i32, y: i32) {
    let mut rng = rand::thread_rng();
    let brightness: i16 = 20;
    let variation: i16 = 15;

    let r = (brightness + rng.gen_range(-variation, variation)) as u8;
    let g = (brightness + rng.gen_range(-variation, variation)) as u8;
    let b = (brightness + rng.gen_range(-variation, variation)) as u8;
    let color = colors::Color {
        r,
        g,
        b,
    };
    world.create_entity()
        .with(Position::new(x,y))
        .with(Renderable::new('.', color))
        .with(Floor{})
        .build();
}