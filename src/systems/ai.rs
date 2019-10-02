use specs::prelude::*;
use shrev::{EventChannel};
use crate::command::{Command, CommandEvent};
use crate::components::{AiControl, MyTurn, Position, Target, Seeing, CanSeeTarget, PlayerControl};
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
    fn get_command(entity: Entity, ai_type: AiType, data: &AiSystemData) -> Command {
        match ai_type {
            AiType::Monster => Monster::get_command(entity, data),
            _ => Command::Move(Dir::Nowhere),
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
    pub command_event_channel:  Write<'a, EventChannel<CommandEvent>>,
    pub my_turns:   WriteStorage<'a, MyTurn>,
    pub world_updater:  Read<'a, LazyUpdate>,
    pub world_resources: ReadExpect<'a, crate::WorldResources>,
    pub view: ReadExpect<'a, View>,
}


impl <'a> System<'a> for Ai {
    type SystemData = AiSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if data.world_resources.player_turn {
            return;
        }

        for (ent, ai_unit, _my_turn) in (&data.entities, &data.ai_units, &data.my_turns).join() {
            let ai_type = ai_unit.ai_type;
            let command = Self::get_command(ent, ai_type, &data);
            let command_event = CommandEvent::new(command, ent);
            data.command_event_channel.single_write(command_event);
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
