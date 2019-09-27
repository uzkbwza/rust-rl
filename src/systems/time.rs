use specs::prelude::*;
use crate::BASE_TURN_TIME;
// use shrev::{EventChannel, ReaderId};
use crate::components::{Actor, MyTurn, PlayerControl};
use crate::components::flags::ActionResult;

// turn structure
// --> TurnAllocator       | finds actors who have 0 fatigue and no MyTurn component, and gives them a MyTurn.
// --> Fatigue             | decrements every actor's fatigue clamped to 0.
// --> StartTurn           | 
// --> PlayerStartTurn     | checks if player has MyTurn and pauses game.
// --> Action              | gets actor actions and removes MyTurn
    // --> Movement, attack, etc... | responsible for actors actually doing stuff on their own accord and generating ActionResults

// --> EndTurn             | iterates through ActionResults, apply cost multiplier to actor's fatigue.
// --> PlayerEndTurn       | checks if player lost MyTurn, resume process.


#[derive(SystemData)]
pub struct TimeSystemData<'a> {
    actors: WriteStorage<'a, Actor>,
    my_turns: WriteStorage<'a, MyTurn>,
    game_state: WriteExpect<'a, crate::GameState>,
}

pub struct Time;
impl<'a> System<'a> for Time {
    type SystemData = TimeSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        if !data.game_state.player_turn {
        }
    }
}

#[derive(SystemData)]
pub struct TurnAllocatorSystemData<'a> {
    entities: Entities<'a>,
    my_turns: WriteStorage<'a, MyTurn>,
    actors: WriteStorage<'a, Actor>,
    game_state: WriteExpect<'a, crate::GameState>,
    action_results: WriteStorage<'a, ActionResult>,


}

pub struct TurnAllocator;
impl<'a> System<'a> for TurnAllocator {
    type SystemData = TurnAllocatorSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        if !data.game_state.player_turn {
            let mut actors = (&data.actors, &data.entities, !&data.my_turns).join().collect::<Vec<_>>();
            actors.sort_by(|&a, &b| a.0.next_turn.cmp(&b.0.next_turn));
            
            data.game_state.world_time.ticks = actors[0].0.next_turn;
            let active_actors = actors.iter().filter(|&a| a.0.next_turn == actors[0].0.next_turn);
            for active in active_actors {
                let active_actor = active.0;
                let active_entity = active.1;
                data.my_turns.insert(active_entity, MyTurn{});
                if active_actor.next_turn >= data.game_state.world_time.ticks {
                    // println!("{:?}", active);
                } else {
                    panic!("Actor next_turn is in the past");
                }
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
}

pub struct EndTurn;
impl<'a> System<'a> for EndTurn {
    type SystemData = EndTurnSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        if !data.game_state.player_turn {
            for (ent, action_result, actor) in (&data.entities, &mut data.action_results, &mut data.actors).join() {
                actor.set_next_turn_from_cost(data.game_state.world_time.ticks, action_result.cost);
                data.world_updater.remove::<ActionResult>(ent);
                // println!("{:?}", actor);
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
            if let Some(_) = data.my_turns.get(ent) {
                return
            }
            data.game_state.player_turn = false;
        }
    }
}