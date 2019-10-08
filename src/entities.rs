use specs::prelude::*;
use crate::components::*;
use crate::systems::ai::types::AiType;
use rand::prelude::*;
use crate::MAP_WIDTH;
use crate::MAP_HEIGHT;
use crate::systems::render::Elevation;


pub fn create_player(world: &mut World, x: i32, y: i32) -> Entity {
    world.create_entity()
        .with(Name::new("Player"))
        .with(Invulnerable{})
        .with(Seeing::new(30))
        .with(Position::new(x,y))
        .with(Quickness::new())
        .with(Renderable::new('@', (255,255,255), None, Elevation::Upright))
        .with(Camera{})
        .with(Corporeal::new(10, 7))
        .with(Actor::from_stats(18, 18, 18))
        .with(PlayerControl{})
        .build()
}

pub fn create_dummy(world: &mut World) -> Entity {
    let mut rng = rand::thread_rng();
    let stats: (u32, u32, u32) = (10,rng.gen_range(2, 13),10);
    
    let x: i32 = rng.gen_range(0, crate::MAP_WIDTH);
    let y: i32 = rng.gen_range(0, crate::MAP_HEIGHT);

    let color = (rng.gen_range(0,255), rng.gen_range(0, 255), rng.gen_range(0, 255));

    let chars = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";
    let random_char = chars
        .chars()
        .choose(&mut rng)
        .unwrap();

    world.create_entity()
//        .with(Name::new("Dummy"))
        .with(Seeing::new(20))
        .with(Quickness::new())
        .with(Position::new(x,y))
        .with(Renderable::new(random_char, color, None, Elevation::Upright))
        .with(Corporeal::new(10, 1))
        .with(Actor::from_stats(stats.0, stats.1, stats.2))
        // .with(Target { entity })
        .with(AiControl { ai_type: AiType::Monster })
        .build()
}

pub fn create_sword(world: &mut World, x: i32, y: i32) {

}

pub fn create_floor(world: &mut World, x: i32, y: i32) {
    let mut rng = rand::thread_rng();
    let brightness: i16 = 25;
    let variation: i16 = 3;
    let chars = ".,`rn ";
    let random_char = chars
        .chars()
        .choose(&mut rng)
        .unwrap();
    let r = (10 + rng.gen_range(0, variation)) as u8;
    let g = (brightness + rng.gen_range(-variation, variation)) as u8;
    let b = (5 + rng.gen_range(0, variation)) as u8;
    let color = (r, g, b );
    let bg_color =  (r - 5, g - 5, b - 5,);

    world.create_entity()
        .with(Position::new(x,y))
        .with(Renderable::new(random_char, color, Some(bg_color), Elevation::Floor))
        .with(Corporeal::new(100, 100))
        .with(Floor{})
        .build();
}

pub fn create_wall(world: &mut World, x: i32, y: i32) {
    world.create_entity()
        .with(Position::new(x, y))
        .with(Renderable::new('#', (255,255,255), Some((100,100,100,)), Elevation::Upright))
        .with(BlockSight)
        .with(BlockMovement{})
        .with(Corporeal::new(100, 100))
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

pub fn create_test_map(world: &mut World) {
    let player = create_player(world, MAP_WIDTH/2, MAP_HEIGHT/2);

    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            create_floor(world, x, y);
        }
    }

    for y in 0..MAP_HEIGHT {
        create_wall(world, MAP_WIDTH - 1, y);
        create_wall(world, 0, y);
    }
    for x in 0..MAP_WIDTH {
        create_wall(world, x, 0);
        create_wall(world, x, MAP_HEIGHT - 1);
    }

    create_shack(world, MAP_WIDTH/2, MAP_HEIGHT/2, 7);

     for _ in 0..50 {
         create_dummy(world);
     }
}