use specs::prelude::*;
use crate::components::*;
use crate::components::flags::ActionResult;
use crate::time::Turn;

#[derive(SystemData)]
pub struct TurnAllocatorSystemData<'a> {
    entities: Entities<'a>,
    my_turns: WriteStorage<'a, MyTurn>,
    actors: WriteStorage<'a, Actor>,
    world_resources: WriteExpect<'a, crate::WorldResources>,
    turn_queue: WriteExpect<'a, crate::time::TurnQueue>,
}

pub struct TurnAllocator;

impl<'a> System<'a> for TurnAllocator {
    type SystemData = TurnAllocatorSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        if !data.world_resources.player_turn {
            for (actor, entity, _my_turn) in (&data.actors, &data.entities, !&data.my_turns).join() {
                let actor_next_turn = Turn {
                    tick: actor.next_turn,
                    entity: entity
                };
                // println!("{}, {}", name.name, actor_next_turn.tick);
                data.turn_queue.push(actor_next_turn);
            }

            let next_turn = data.turn_queue.peek().unwrap().tick;
            // println!("{:?}", data.turn_queue.peek());

            // loop through all "next turns" that store the same tick, making sure all actors who are ready
            // on the same turn get to act on the same gameloop iteration (unordered) 
            while !data.turn_queue.is_empty() && data.turn_queue.peek().unwrap().tick == next_turn {
                let turn = data.turn_queue.pop().unwrap();
                assert_eq!(next_turn, turn.tick);
                data.world_resources.world_time.tick = turn.tick;
                data.world_resources.world_time.determine_world_turn();
                if let Err(err) = data.my_turns.insert(turn.entity, MyTurn{}) {
                    error!("Failed to insert turn: {}", err)
                }
                // println!("turn queue length: {:?}", data.turn_queue.len());
            }
        }
    }
}

#[derive(SystemData)]
pub struct PlayerStartTurnSystemData<'a> {
    entities: Entities<'a>,
    world_resources: WriteExpect<'a, crate::WorldResources>,
    my_turns: ReadStorage<'a, MyTurn>,
    players: ReadStorage<'a, PlayerControl>,
}

pub struct PlayerStartTurn;
impl<'a> System<'a> for PlayerStartTurn {
    type SystemData = PlayerStartTurnSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        for (ent, _player) in (&data.entities, &data.players).join() {
            if let Some(_my_turn) = &data.my_turns.get(ent) {
                data.world_resources.player_turn = true;
            } else {
                data.world_resources.player_turn = false;
            }
        }
    }
}

#[derive(SystemData)]
pub struct EndTurnSystemData<'a> {
    entities: Entities<'a>,
    actors: WriteStorage<'a, Actor>,
    action_results: WriteStorage<'a, ActionResult>,
    _world_updater: Read<'a, LazyUpdate>,
    world_resources: WriteExpect<'a, crate::WorldResources>,
    _turn_queue: WriteExpect<'a, crate::time::TurnQueue>,
    players: ReadStorage<'a, PlayerControl>,
}

pub struct EndTurn;
impl<'a> System<'a> for EndTurn {
    type SystemData = EndTurnSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        if !data.world_resources.player_turn {
            for (ent, actor) in (&data.entities, &mut data.actors).join() {
                if let Some(result) = data.action_results.get_mut(ent) {
                    actor.set_next_turn_from_cost(data.world_resources.world_time.tick, result.cost);
                    if let Some(_) = data.players.get(ent) {
                        // println!("{}", data.world_resources.world_time.tick);
                        // println!("{:?} next turn set to {:?} from cost {:?}", ent, actor.next_turn, result.cost);
                    }
                    data.action_results.remove(ent);
                }
            }
        }
    }
}
