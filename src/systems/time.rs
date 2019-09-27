use specs::prelude::*;
use crate::BASE_TURN_TIME;
// use shrev::{EventChannel, ReaderId};
use crate::components::*;
use crate::components::flags::ActionResult;
use crate::Turn;


// turn structure
// --> TurnAllocator       | finds actors who have 0 fatigue and no MyTurn component, and gives them a MyTurn.
// --> Fatigue             | decrements every actor's fatigue clamped to 0.
// --> StartTurn           | 
// --> PlayerStartTurn     | checks if player has MyTurn and pauses game.
// --> Action              | gets actor actions and removes MyTurn
    // --> Movement, attack, etc... | responsible for actors actually doing stuff on their own accord and generating ActionResults

// --> EndTurn             | iterates through ActionResults, apply cost multiplier to actor's fatigue.
// --> PlayerEndTurn       | checks if player lost MyTurn, resume process.


// #[derive(SystemData)]
// pub struct TimeSystemData<'a> {
//     actors: WriteStorage<'a, Actor>,
//     my_turns: WriteStorage<'a, MyTurn>,
//     game_state: WriteExpect<'a, crate::GameState>,
// }

// pub struct Time;
// impl<'a> System<'a> for Time {
//     type SystemData = TimeSystemData<'a>;
//     fn run(&mut self, mut data: Self::SystemData) {
//         if !data.game_state.player_turn {
//         }
//     }
// }

#[derive(SystemData)]
pub struct TurnAllocatorSystemData<'a> {
    entities: Entities<'a>,
    names: ReadStorage<'a, Name>,
    my_turns: WriteStorage<'a, MyTurn>,
    actors: WriteStorage<'a, Actor>,
    game_state: WriteExpect<'a, crate::GameState>,
    action_results: WriteStorage<'a, ActionResult>,
    turn_queue: WriteExpect<'a, crate::TurnQueue>,
}

pub struct TurnAllocator;

impl<'a> System<'a> for TurnAllocator {
    type SystemData = TurnAllocatorSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        if !data.game_state.player_turn {
            for (actor, entity, _my_turn) in (&data.actors, &data.entities, !&data.my_turns).join() {
                let actor_next_turn = Turn {
                    tick: actor.next_turn,
                    entity: entity
                };
                data.turn_queue.push(actor_next_turn);
            }

            let next_turn = data.turn_queue.peek().unwrap().tick;
            while !data.turn_queue.is_empty() && data.turn_queue.peek().unwrap().tick == next_turn {
                let turn = data.turn_queue.pop().unwrap();
                assert_eq!(next_turn, turn.tick);
                data.game_state.world_time.tick = turn.tick;
                data.game_state.world_time.determine_world_turn();
                data.my_turns.insert(turn.entity, MyTurn{});
                println!("{:?}", data.turn_queue.len());
            }
        }
    }
}

#[derive(SystemData)]
pub struct PlayerStartTurnSystemData<'a> {
    entities: Entities<'a>,
    game_state: WriteExpect<'a, crate::GameState>,
    my_turns: ReadStorage<'a, MyTurn>,
    players: ReadStorage<'a, PlayerControl>,
}

pub struct PlayerStartTurn;
impl<'a> System<'a> for PlayerStartTurn {
    type SystemData = PlayerStartTurnSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        for (ent, _player) in (&data.entities, &data.players).join() {
            if let Some(_my_turn) = &data.my_turns.get(ent) {
                data.game_state.player_turn = true;
            } else {
                data.game_state.player_turn = false;
            }
        }
    }
}

#[derive(SystemData)]
pub struct EndTurnSystemData<'a> {
    entities: Entities<'a>,
    actors: WriteStorage<'a, Actor>,
    action_results: WriteStorage<'a, ActionResult>,
    my_turns: WriteStorage<'a, MyTurn>,
    world_updater: Read<'a, LazyUpdate>,
    game_state: WriteExpect<'a, crate::GameState>,
    turn_queue: WriteExpect<'a, crate::TurnQueue>,
}

pub struct EndTurn;
impl<'a> System<'a> for EndTurn {
    type SystemData = EndTurnSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        if !data.game_state.player_turn {
            for (ent, actor) in (&data.entities, &mut data.actors).join() {
                if let Some(result) = data.action_results.get_mut(ent) {
                    actor.set_next_turn_from_cost(data.game_state.world_time.tick, result.cost);
                    // println!("actor next turn set to {:?} from cost {:?}", actor.next_turn, result.cost);
                    data.action_results.remove(ent);
                }
            }
        }
    }
}

#[derive(SystemData)]
pub struct PlayerEndTurnSystemData<'a> {
    entities: Entities<'a>,
    actors: WriteStorage<'a, Actor>,
    action_results: WriteStorage<'a, ActionResult>,
    players: ReadStorage<'a, PlayerControl>,
    my_turns: WriteStorage<'a, MyTurn>,
    world_updater: Read<'a, LazyUpdate>,
    game_state: WriteExpect<'a, crate::GameState>,
}

pub struct PlayerEndTurn;
impl<'a> System<'a> for PlayerEndTurn {
    type SystemData = PlayerEndTurnSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        for (ent, player) in (&data.entities, &data.players).join() {

        }
    }
}