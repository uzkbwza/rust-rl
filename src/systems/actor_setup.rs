use specs::prelude::*;
use crate::components::*;
use crate::systems::ai::types::AiType;

#[derive(SystemData)]
pub struct ActorSetupSystemData<'a> {
    pub entities: Entities<'a>,
    pub actors: ReadStorage<'a, Actor>,
    pub command_sequences: WriteStorage<'a, CommandSequence>,
}

pub struct ActorSetup;
impl<'a> System<'a> for ActorSetup {
    type SystemData = ActorSetupSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (ent, actor) in (&data.entities, &data.actors).join() {
            if data.command_sequences.get(ent) == None {
                data.command_sequences.insert(ent, CommandSequence::default());
            }
        }
    }
}