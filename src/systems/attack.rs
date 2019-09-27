use specs::prelude::*;
use shrev::{EventChannel, ReaderId};
use crate::map::EntityMap;
use crate::components::*;
use crate::components::flags::requests::*;
use crate::map::View;
use crate::systems::movement::Dir;
use crate::MessageLog;
use crate::components::flags::ActionResult;
use crate::BASE_TURN_TIME;


pub struct Attack;

#[derive(SystemData)]
pub struct AttackSystemData<'a> {
    pub entities: Entities<'a>,
    pub actors: ReadStorage<'a, Actor>,
    pub attack_requests: WriteStorage<'a, AttackRequest>,
    pub action_results: WriteStorage<'a, ActionResult>,
    pub world_updater: Read<'a, LazyUpdate>,
    pub positions: ReadStorage<'a, Position>,
    pub corporeals: WriteStorage<'a, Corporeal>,
    pub floors: ReadStorage<'a, Floor>,
    pub message_log: WriteExpect<'a, MessageLog>,
    pub names: ReadStorage<'a, Name>,
}

impl<'a> System<'a> for Attack {
    type SystemData = AttackSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (ent, pos, name, attack_request) in (&data.entities, &data.positions, &data.names,  &mut data.attack_requests).join() {
            data.world_updater.remove::<AttackRequest>(ent);
            let attack_pos = Dir::dir_to_pos(attack_request.dir); 
            let attack_pos = Position::new(pos.x + attack_pos.0, pos.y + attack_pos.1);
            for (target_ent, target_pos, target_name, corporeal, _floor) in (&data.entities, &data.positions, &data.names, &mut data.corporeals, !&data.floors).join() {
                if *target_pos == attack_pos {
                    // println!("attacking", );
                    data.message_log.log(format!("{} attacks {}", name.name, target_name.name));
                }
            }
            data.action_results.insert(ent, ActionResult::from(BASE_TURN_TIME));
        }
    }
}