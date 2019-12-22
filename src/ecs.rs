
use crate::components::{Position, Renderable};
use crate::entity_factory::{EntityBlueprint, EntityFactory, EntityLoadQueue};
use crate::map;
use crate::systems;
use crate::systems::render::Tile;
use crate::time;
use crate::CONFIG;
use rltk::RandomNumberGenerator;
use specs::prelude::*;
use std::sync::{Arc, Mutex};
use systems::render::LayeredTileMap;
use tcod::console::*;
use tcod::map::Map as TcodMap;
use vecmap::*;
use std::collections::HashMap;
use shrev::EventChannel;
use crate::command::CommandEvent;
use tcod::input::Key;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum State {
    MapGen,
    TurnProcess,
    PlayerTurn,
}

pub struct GameState {
    current_state: State,
    pub game_end: bool,
    pub world_time: time::WorldTime,
}

impl GameState {
    pub fn transition(&mut self, state: State) {
        println!("{:?}", state);
        self.current_state = state;
    }

    pub fn current(&self) -> State {
        self.current_state
    }
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
            _ => Some(self.messages.remove(0)),
        }
    }
}

pub struct Ecs {
    world: World,
    dispatchers: HashMap<State, Dispatcher<'static, 'static>>,
}

impl Ecs {
    pub fn main_loop(&mut self) {
        let mut factory = EntityFactory::new("blueprints");
        let mut current_state = None;
        loop {
            self.world.maintain();
            let mut blueprints: Vec<EntityBlueprint> = Vec::new();
            self.build_blueprints(&mut blueprints, &mut factory);
            {
                let game_state = self.world.read_resource::<GameState>();
                let mut root = self.world.write_resource::<Root>();


                root.flush();

                if root.window_closed() || game_state.game_end {
                    break;
                }

                current_state = Some(game_state.current_state);
            }
            let mut dispatcher = self.dispatchers
                .get_mut(&current_state.unwrap())
                .expect("Could not get dispatcher for state");

            dispatcher.dispatch(&mut self.world);
        }
    }

    pub fn build_blueprints(
        &mut self,
        blueprints: &mut Vec<EntityBlueprint>,
        factory: &mut EntityFactory,
    ) {
        let mut blueprint_queue = &mut self.world.write_resource::<EntityLoadQueue>().clone();

        if blueprint_queue.is_empty() {
            return;
        }
        println!("# of blueprints to build: {}", blueprint_queue.len());
        for _ in 0..blueprint_queue.len() {
            let blueprint = blueprint_queue.pop().unwrap();
            factory.build(blueprint.0, &mut self.world, blueprint.1);
        }

        self.world.write_resource::<EntityLoadQueue>().clear();
    }
}

//pub struct
pub fn world_setup<'a, 'b>() -> Ecs {
    //    println!("{:?}", CONFIG);
    let mut world = World::new();

    let mut dispatchers = HashMap::new();

    let map_gen_dispatcher = DispatcherBuilder::new()
        .with(systems::mapgen::MapGen::new(), "map_gen_sys", &[])
        .build();

    let player_turn_dispatcher = DispatcherBuilder::new()
        .with(systems::render::RandomRender, "random_render_sys", &[])
        .with(systems::input::InputListener, "input_listener_sys", &[])
        .with(
            systems::input::Input::new(),
            "input_sys",
            &["input_listener_sys"],
        )
        .with(
            systems::render::RenderViewport::new(),
            "render_viewport_sys",
            &[],
        )
        .with(systems::render::RenderUi, "render_ui_sys", &[])
        .with(systems::naming::Naming, "naming_sys", &[])
        .with(systems::actor_setup::ActorSetup, "actor_setup_sys", &[])
        .with(
            systems::movement::CollisionMapUpdater::new(),
            "collision_map_updater_sys",
            &[],
        )
        .with_barrier()
        .with(systems::ai::Ai, "ai_sys", &[])
        .with(
            systems::action::ActionHandler::new(),
            "action_sys",
            &["ai_sys"],
        )
        .with(systems::time::EndTurn, "end_turn_sys", &[])
        .build();

    let turn_process_dispatcher = DispatcherBuilder::new()
        .with(systems::render::RandomRender, "random_render_sys", &[])
        .with(systems::input::InputListener, "input_listener_sys", &[])
        .with(
            systems::input::Input::new(),
            "input_sys",
            &["input_listener_sys"],
        )
        .with(
            systems::render::RenderViewport::new(),
            "render_viewport_sys",
            &[],
        )
        .with(systems::render::RenderUi, "render_ui_sys", &[])
        .with(systems::naming::Naming, "naming_sys", &[])
        .with(systems::actor_setup::ActorSetup, "actor_setup_sys", &[])
        .with(
            systems::movement::CollisionMapUpdater::new(),
            "collision_map_updater_sys",
            &[],
        )
        .with_barrier()
        .with(systems::ai::Ai, "ai_sys", &[])
        .with(systems::time::TurnAllocator, "turn_allocator_sys", &[])
        .with(systems::stats::QuicknessSystem, "quickness_sys", &[])
        .with(
            systems::action::ActionHandler::new(),
            "action_sys",
            &["ai_sys"],
        )
        .with(
            systems::movement::Movement,
            "movement_sys",
            &["action_sys", "collision_map_updater_sys"],
        )
        .with(systems::combat::DeathSystem, "death_sys", &[])
        .with(
            systems::combat::Attack,
            "attack_sys",
            &["death_sys", "movement_sys", "action_sys"],
        )
        .with(systems::combat::Defend, "defend_sys", &["attack_sys"])
        //        .with_barrier()
        .with(systems::time::EndTurn, "end_turn_sys", &[])
        .build();

    dispatchers.insert(State::MapGen, map_gen_dispatcher);
    dispatchers.insert(State::TurnProcess, turn_process_dispatcher);
    dispatchers.insert(State::PlayerTurn, player_turn_dispatcher);


    for dispatcher in dispatchers.values_mut() {
        dispatcher.setup(&mut world);
    }

    let world_time = time::WorldTime::new();
    let game_state = GameState {
        current_state: State::MapGen,
        game_end: false,
        world_time,
    };

    let view = map::View {
        map: Arc::new(Mutex::new(TcodMap::new(
            CONFIG.map_width,
            CONFIG.map_height,
        ))),
        block_map: VecMap::filled_with(
            map::BlockTile::default(),
            CONFIG.map_width,
            CONFIG.map_height,
        ),
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

    // insert event channels
    let command_event_channel: EventChannel<CommandEvent> = EventChannel::new();

    world.insert(command_event_channel);

    // insert readers
    let key_reader = world.fetch_mut::<EventChannel<Key>>().register_reader();
    let command_event_reader = world.fetch_mut::<EventChannel<CommandEvent>>()
        .register_reader();

    world.insert(command_event_reader);
    world.insert(key_reader);

//    let dispatcher = dispatchers
//        .get_mut(&game_state.current_state)
//        .unwrap();
//
//    dispatcher.dispatch(&mut world);

    Ecs { world, dispatchers }
}

