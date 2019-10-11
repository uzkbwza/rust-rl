use specs::prelude::*;
use shrev::{EventChannel};
use crate::command::{Command, CommandEvent};
use crate::components::*;
use crate::systems::movement::{Dir};
use crate::map::{EntityMap, View};
pub mod types;
pub mod thinking;
mod pathfinding;
use types::AiType;
use types::monster::*;
use thinking::Thinking;

// use std::sync::{Arc, Mutex};

pub struct Ai;
impl Ai {
    fn get_command(entity: Entity, ai_type: AiType, data: &AiSystemData) -> Vec<Command> {
        match ai_type {
            AiType::Monster => Monster::get_command(entity, data),
            _ => vec![Command::Move(Dir::Nowhere)],
        }
    }
}

#[derive(SystemData)]
pub struct AiSystemData<'a> {
    pub entities: Entities<'a>,
    pub players: ReadStorage<'a, PlayerControl>,
    pub entity_map: ReadExpect<'a, EntityMap>,
    pub sees_targets: ReadStorage<'a, CanSeeTarget>,
    pub positions:  ReadStorage<'a, Position>,
    pub targets:     WriteStorage<'a, Target>,
    pub ai_units:    ReadStorage<'a, AiControl>,
    pub seers: ReadStorage<'a, Seeing>,
    pub my_turns:   WriteStorage<'a, MyTurn>,
    pub world_updater:  Read<'a, LazyUpdate>,
    pub game_state: ReadExpect<'a, crate::GameState>,
    pub command_event_channel:  Write<'a, EventChannel<CommandEvent>>,
    pub view: ReadExpect<'a, View>,
    pub actors: WriteStorage<'a, Actor>,
}


impl <'a> System<'a> for Ai {
    type SystemData = AiSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if data.game_state.player_turn {
            return;
        }

        // we need to get the commands from all actors, stuff it in here, and then process it later.
        // the reason there are two loops is because we need to keep data immutable in the first one,
        // but we need to mutably access it to update the actors' remaining moves, so the second one
        // actually sends out the command events.
        let mut commands = Vec::new();

        for (ent, ai_unit, actor, _my_turn) in (&data.entities, &data.ai_units, &data.actors, &data.my_turns).join() {
            let mut command_sequence: Vec<Command> = Vec::new();
            let mut command_event: Option<CommandEvent> = None;

            let mut command = Command::Move(Dir::Nowhere);
            let ai_type = ai_unit.ai_type;

            // if nothing in command sequence,
            command_sequence = match actor.command_sequence.is_empty() {
                true => { Self::get_command(ent, ai_type, &data ) },
                false => { actor.command_sequence.clone() },
            };

            println!("{:?}", &command_sequence);

            command_event = Some(CommandEvent::new(command_sequence.pop().unwrap(), ent));
            commands.push((command_sequence, command_event));

        }

        for (command_sequence, command_event) in commands {
            if let Some(ce) = command_event {
                if let Some(actor) = data.actors.get_mut(ce.entity) {
                    actor.command_sequence = command_sequence.clone();
                }
                data.command_event_channel.single_write(ce);
            }
        }

        for (entity, target, _sees_target, _my_turn) in (&data.entities, &mut data.targets, !&data.sees_targets, &data.my_turns).join() {
            target.decrement_timer();
            // println!("{}", target.give_up_timer);
            if target.give_up_timer <= 0 {
                data.world_updater.remove::<Target>(entity);
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        let command_event_channel: EventChannel<CommandEvent> = EventChannel::new();
        world.insert(command_event_channel);
    }
}
