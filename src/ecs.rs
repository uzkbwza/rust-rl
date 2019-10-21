use specs::prelude::*;
use crate::time;
use crate::map;
use crate::systems;
use crate::entities;
use rltk::RandomNumberGenerator;
use std::sync::{Arc, Mutex};
use tcod::map::Map as TcodMap;
use tcod::console::*;
use vecmap::*;
use crate::systems::render::Tile;
use crate::CONFIG;
use crate::entity_factory::{EntityBlueprint, EntityLoadQueue, EntityFactory};
use crate::components::Position;
use systems::render::LayeredTileMap;


pub struct GameState {
    pub player_turn: bool,
    pub real_time: bool,
    pub game_end: bool,
    pub world_time: time::WorldTime,
}

pub struct MessageLog {
    pub messages: Vec<String>,
}

impl MessageLog {
    pub fn new() -> Self {
        MessageLog {
            messages: Vec::new(),
        }
    }

    pub fn log(&mut self, string: String) {
        self.messages.insert(0, string);
    }

    pub fn pop(&mut self) -> Option<String> {
        match self.messages.len() {
            0 => None,
            _ => Some(self.messages.remove(0))
        }
    }
}

pub struct Ecs {
    world: World,
    dispatcher: Dispatcher<'static, 'static>,
}

impl Ecs {
    pub fn main_loop(&mut self) {
        let mut factory = EntityFactory::new("blueprints");
        loop {
            self.world.maintain();
            let mut blueprints : Vec<EntityBlueprint> = Vec::new();
            self.build_blueprints(&mut blueprints, &mut factory);
            {
                let game_state = self.world.read_resource::<GameState>();
                let mut root = self.world.write_resource::<Root>();
                root.flush();
                if root.window_closed() || game_state.game_end { break }
            }
            self.dispatcher.dispatch(&mut self.world);
        }
    }

    pub fn build_blueprints(&mut self, blueprints: &mut Vec<EntityBlueprint>, factory: &mut EntityFactory) {
        let mut blueprint_queue = &mut self.world.write_resource::<EntityLoadQueue>().clone();

        if blueprint_queue.is_empty() { return }
        println!("{}",blueprint_queue.len());
        for _ in 0..blueprint_queue.len() {
            let blueprint = blueprint_queue.pop().unwrap();
            factory.build(blueprint.0, &mut self.world, blueprint.1);
        }

        self.world.write_resource::<EntityLoadQueue>().clear();

    }
}

//pub struct
pub fn world_setup<'a, 'b> () -> Ecs {
//    println!("{:?}", CONFIG);
    let mut world = World::new();
    let builder = DispatcherBuilder::new()
        .with(systems::render::RandomRender, "random_render_sys", &[])
        .with(systems::mapgen::MapGen::new(), "map_gen_sys", &[])
        .with(systems::naming::Naming, "naming_sys", &[])
        .with(systems::actor_setup::ActorSetup, "actor_setup_sys", &[])
        .with(systems::movement::CollisionMapUpdater::new(), "collision_map_updater_sys", &[])
        .with_barrier()
        .with(systems::input::InputListener, "input_listener_sys", &[])
        .with(systems::ai::Ai, "ai_sys", &[])
        .with(systems::time::TurnAllocator, "turn_allocator_sys", &[])
        .with(systems::stats::QuicknessSystem, "quickness_sys", &[])
//        .with_barrier()
        .with(systems::input::Input::new(), "input_sys", &[])
        .with(systems::action::ActionHandler::new(), "action_sys", &["ai_sys"])
        .with(systems::movement::Movement, "movement_sys", &["action_sys", "collision_map_updater_sys"])
        .with(systems::combat::DeathSystem, "death_sys", &[])
        .with(systems::combat::Attack, "attack_sys", &["death_sys", "movement_sys", "action_sys"])
        .with(systems::combat::Defend, "defend_sys", &["attack_sys"])
//        .with_barrier()
        .with(systems::time::EndTurn, "end_turn_sys", &[])
        .with(systems::render::RenderViewport::new(), "render_viewport_sys", &[])
        .with(systems::render::RenderUi, "render_ui_sys", &[]);

    let mut dispatcher = builder.build();
    dispatcher.setup(&mut world);

    let world_time = time::WorldTime::new();
    let game_state = GameState {
        player_turn: false,
        real_time: false,
        game_end: false,
        world_time,
    };

    let view = map::View {
        map: Arc::new(Mutex::new(TcodMap::new(CONFIG.map_width, CONFIG.map_height))),
        block_map: VecMap::filled_with(map::BlockTile::default(), CONFIG.map_width, CONFIG.map_height)
    };
    let map = map::EntityMap::new(CONFIG.map_width as usize, CONFIG.map_height as usize);
    let message_log = MessageLog::new();
    let root = Root::initializer()
        .size(CONFIG.screen_width, CONFIG.screen_height)
        .font("term.png", FontLayout::AsciiInRow)
        .init();

    world.insert(game_state);
    world.insert(map);
    world.insert(view);
    world.insert(message_log);
    world.insert(time::TurnQueue::new());
    world.insert(LayeredTileMap::new(CONFIG.map_width, CONFIG.map_height));
    world.insert(RandomNumberGenerator::new());
    world.insert(root);
    world.insert(EntityLoadQueue::new());

    entities::create_test_map(&mut world);

    dispatcher.dispatch(&mut world);

    Ecs { world, dispatcher }
}

