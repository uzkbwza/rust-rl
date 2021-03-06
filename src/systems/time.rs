use crate::components::flags::ActionResult;
use crate::components::*;
use crate::time::Turn;
use crate::CONFIG;
use specs::prelude::*;
use crate::ecs::State;

#[derive(SystemData)]
pub struct TurnAllocatorSystemData<'a> {
    entities: Entities<'a>,
    my_turns: WriteStorage<'a, MyTurn>,
    actors: WriteStorage<'a, Actor>,
    players: ReadStorage<'a, PlayerControl>,
    game_state: WriteExpect<'a, crate::GameState>,
    turn_queue: WriteExpect<'a, crate::time::TurnQueue>,
    message_log: WriteExpect<'a, crate::MessageLog>,
}

pub struct TurnAllocator;

impl<'a> System<'a> for TurnAllocator {
    type SystemData = TurnAllocatorSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        for (actor, entity, _my_turn) in (&data.actors, &data.entities, !&data.my_turns).join()
        {
            let actor_next_turn = Turn {
                tick: actor.next_turn,
                entity: entity,
            };
            // println!("{}, {}", name.name, actor_next_turn.tick);
            data.turn_queue.push(actor_next_turn);
        }

        if data.turn_queue.is_empty() {
            return;
        }

        let next_turn = data.turn_queue.peek().unwrap().tick;

        // loop through all "next turns" that store the same tick, making sure all actors who are ready
        // on the same turn get to act on the same gameloop iteration (unordered)
        while !data.turn_queue.is_empty() && data.turn_queue.peek().unwrap().tick == next_turn {
            let turn = data.turn_queue.pop().unwrap();
            assert_eq!(next_turn, turn.tick);
            data.game_state.world_time.tick = turn.tick;
            data.game_state.world_time.determine_world_turn();
            if let Err(err) = data.my_turns.insert(turn.entity, MyTurn {}) {
                error!("Failed to insert turn: {}", err)
            }

            if let Some(_) = data.players.get(turn.entity) {
                if data.game_state.current() == State::TurnProcess {
                    data.game_state.transition(State::PlayerTurn);
                    if CONFIG.log_turn_start {
                        data.message_log.log(String::from("[TURN START]"));
                    }
                }
            }
            // println!("turn queue length: {:?}", data.turn_queue.len());
        }
    }
}

#[derive(SystemData)]
pub struct EndTurnSystemData<'a> {
    entities: Entities<'a>,
    actors: WriteStorage<'a, Actor>,
    action_results: WriteStorage<'a, ActionResult>,
    game_state: WriteExpect<'a, crate::GameState>,
    players: ReadStorage<'a, PlayerControl>,
}

pub struct EndTurn;
impl<'a> System<'a> for EndTurn {
    type SystemData = EndTurnSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        for (ent, actor) in (&data.entities, &mut data.actors).join() {
            if let Some(result) = data.action_results.get_mut(ent) {
                actor.set_next_turn_from_cost(data.game_state.world_time.tick, result.cost);
                if let Some(_) = data.players.get(ent) {
                    // println!("{}", data.game_state.world_time.tick);
                    // println!("{:?} next turn set to {:?} from cost {:?}", ent, actor.next_turn, result.cost);
                }
                data.action_results.remove(ent);
            }
        }
    }
}
