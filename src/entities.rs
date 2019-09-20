use specs::prelude::*;
use crate::components::*;
use crate::systems::ai::types::AiType;
use rand::prelude::*;
use tcod::colors;
use tcod::chars;

pub fn create_player(world: &mut World, x: i32, y: i32) -> Entity {
    world.create_entity()
        .with(Name::new("Player"))
        .with(Seeing::new(30))
        .with(Position::new(x,y))
        .with(Renderable::new('@', colors::WHITE, None))
        .with(Camera{})
        .with(CostMultiplier { multiplier: 1.0 })
        .with(Corporeal::new(10))
        .with(Actor::from_stats(18, 18, 18))
        .with(PlayerControl{})
        .build()
}

pub fn create_dummy(world: &mut World, entity: Entity) -> Entity {
    let mut rng = rand::thread_rng();
    let stats: (i32, i32, i32) = (10,rng.gen_range(10, 14),10);
    let x: i32 = rng.gen_range(0, crate::MAP_WIDTH);
    let y: i32 = rng.gen_range(0, crate::MAP_HEIGHT);


    let color = colors::Color::new(rng.gen_range(0,255), rng.gen_range(0, 255), rng.gen_range(0, 255));
    
    let chars = "obcdfsrxvlgZhq";
    let random_char = chars
        .chars()
        .choose(&mut rng)
        .unwrap();

    world.create_entity()
        .with(Name::new("Dummy"))
        .with(Seeing::new(20))
        .with(Position::new(x,y))
        .with(Renderable::new(random_char, color, None))
        .with(Corporeal::new(10))
        .with(CostMultiplier { multiplier: 1.0 })
        .with(Actor::from_stats(stats.0, stats.1, stats.2))
        // .with(Target { entity })
        .with(AiControl { ai_type: AiType::Monster })
        .build()
}

pub fn create_floor(world: &mut World, x: i32, y: i32) {
    let mut rng = rand::thread_rng();
    let brightness: i16 = 20;
    let variation: i16 = 5;
    let chars = "rn.,` ";
    let random_char = chars
        .chars()
        .choose(&mut rng)
        .unwrap();
    let r = (5 + rng.gen_range(0, variation)) as u8;
    let g = (brightness + rng.gen_range(-variation, variation)) as u8;
    let b = (5 + rng.gen_range(0, variation)) as u8;
    let color = colors::Color {
        r,
        g,
        b,
    };

    let bg_color = colors::Color {
        r: r - 5,
        g: g - 5,
        b: b - 5,

    };

    world.create_entity()
        .with(Position::new(x,y))
        // .with(PlayerControl{})
        // .with(Actor::new())
        // .with(Stats::new(10,16,10))
        .with(Renderable::new(random_char, color, Some(bg_color)))
        .with(Floor{})
        .build();
}

pub fn create_wall(world: &mut World, x: i32, y: i32) {
    world.create_entity()
        .with(Position::new(x, y))
        .with(Renderable::new('#', colors::WHITE, Some(colors::DARK_GREY)))
        .with(BlockSight)
        .with(BlockMovement{})
        .with(Collidable)
        .build();
}

pub fn create_shack(world: &mut World, center_x: i32, center_y: i32, size: i32) {
    for i in 0..size*2+1 {
        create_wall(world, center_x + size - i, center_y + size);
        create_wall(world, center_x + size, center_y + size - i);
        create_wall(world, center_x - size, center_y + size - i);
        if i != size {
            create_wall(world, center_x + size - i, center_y - size)
        }
    }
}