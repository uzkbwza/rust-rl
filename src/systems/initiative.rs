use specs::prelude::*;
// use shrev::{EventChannel, ReaderId};
use crate::components::{Actor, CostMultiplier, Stats, MyTurn, PlayerControl};

// todo: make actual initiative values a little more procedural and meaningful 
const MAX_FATIGUE: i32 = 2500;

#[derive(SystemData)]
pub struct InitiativeSystemData<'a> {
    entities: Entities<'a>,
    actors: WriteStorage<'a, Actor>,
    cost_multipliers: WriteStorage<'a, CostMultiplier>,
    players: ReadStorage<'a, PlayerControl>,
    my_turns: WriteStorage<'a, MyTurn>,
    stats_lists: ReadStorage<'a, Stats>,
    world_updater: Read<'a, LazyUpdate>,
    game_state: WriteExpect<'a, crate::GameState>,

}

pub struct Initiative;
impl Initiative {
    // some magic numbers in the player stats rn considering they dont do anything yet. will hopefully
    // flesh them out a bit more
    fn get_initiative_from_agility(agility: i32) -> i32 {
        (agility * 100)
    }
}   

impl<'a> System<'a> for Initiative {
    type SystemData = InitiativeSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        if !data.game_state.player_turn {
            for (ent, actor, _my_turn) in (&data.entities, &mut data.actors, !&data.my_turns).join() {
                if actor.fatigue > 0 {
                    let speed = MAX_FATIGUE / 10;
                    actor.decrement_fatigue(speed);
                } else {
                    if let Some(stats_list) = data.stats_lists.get(ent) {
                        actor.fatigue = MAX_FATIGUE - Self::get_initiative_from_agility(stats_list.agility);
                        if let Some(cost_multiplier) = &mut data.cost_multipliers.get_mut(ent) {
                            actor.fatigue = (actor.fatigue as f32 * cost_multiplier.multiplier) as i32;
                        }
                    } else {
                        actor.fatigue = MAX_FATIGUE;
                    }
                    data.world_updater.insert(ent, MyTurn);

                    // if Player gets MyTurn component, pause the game
                    if let Some(_player) = data.players.get(ent) {
                        data.game_state.player_turn = true
                    }
                }
            }
        }
    }
}