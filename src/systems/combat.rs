use specs::prelude::*;
use crate::components::*;
use crate::components::flags::requests::*;
use crate::systems::movement::Dir;
use crate::ecs::MessageLog;
use crate::components::flags::*;
use crate::CONFIG;
use crate::components::Elevation;
use crate::map::*;

pub struct Attack;

// TODO: refactor the shit out of this
#[derive(SystemData)]
pub struct CombatSystemData<'a> {
    pub entities: Entities<'a>,
    pub actors: WriteStorage<'a, Actor>,
    pub attack_requests: WriteStorage<'a, AttackRequest>,
    pub action_results: WriteStorage<'a, ActionResult>,
    pub world_updater: Read<'a, LazyUpdate>,
    pub positions: ReadStorage<'a, Position>,
    pub floors: ReadStorage<'a, Floor>,
    pub message_log: WriteExpect<'a, MessageLog>,
    pub names: WriteStorage<'a, Name>,
    pub mobiles: ReadStorage<'a, Mobile>,
    pub corporeals: WriteStorage<'a, Corporeal>,
    pub defenders: WriteStorage<'a, Defending>,
    pub invulnerables: ReadStorage<'a, Invulnerable>,
    pub bodies: WriteStorage<'a, Body>,
    pub deaths: WriteStorage<'a, Death>,
    pub corpses: WriteStorage<'a, Corpse>,
    pub entity_map: WriteExpect<'a, EntityMap>,
    pub renderables: WriteStorage<'a, Renderable>,
    pub move_requests: WriteStorage<'a, MoveRequest>,
    pub ai_units: WriteStorage<'a, AiControl>,
    pub view: WriteExpect<'a, View>,
    pub players: ReadStorage<'a, PlayerControl>,
    pub elevations: WriteStorage<'a, Elevation>,

}

impl Attack {
    fn get_cost(base: u32, modifier: f32) -> u32 {
        (modifier * base as f32) as u32
    }
}

impl<'a> System<'a> for Attack {
    type SystemData = CombatSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (ent, pos, name, corporeal, attack_request) in (&data.entities, &data.positions, &data.names, &data.corporeals, &mut data.attack_requests).join() {
            data.world_updater.remove::<AttackRequest>(ent);
            let attack_pos = Dir::dir_to_pos(attack_request.dir);
            let attack_pos = Position::new(pos.x + attack_pos.0, pos.y + attack_pos.1);
            let attack_damage = corporeal.base_damage;

            for (target_entity, target_pos, target_name, corporeal, _floor) in (&data.entities, &data.positions, &data.names, &data.corporeals, !&data.floors).join() {
                // don't do anything when entity attacks empty space
                if *target_pos == attack_pos {
                    data.message_log.log(format!("{} attempts to attack {}!", name.name, target_name.name));
                    data.defenders.insert(target_entity, Defending {
                        damage_source: ent,
                        damage_amount: attack_damage,
                    });
                }
            }

            let cost = match data.mobiles.get(ent) {
                Some(quickness) => Self::get_cost(quickness.quickness, 1.0),
                None => Self::get_cost(CONFIG.base_turn_time, 1.0),
            };

            if let Err(err) = data.action_results.insert(ent, ActionResult::from(cost)) {
                error!("Failed to insert action result from Attack system: {}", err)
            }
        }
    }
}

pub struct Defend;
impl<'a> System<'a> for Defend {
    type SystemData = CombatSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (ent, pos, name, mut corporeal, defender) in (&data.entities, &data.positions, &data.names, &mut data.corporeals, &mut data.defenders).join() {
            data.world_updater.remove::<Defending>(ent);
            let dmg = defender.damage_amount;
            let attacker = defender.damage_source;
            let mut attacker_name = match data.names.get(attacker) {
                Some(name) => name.name.clone(),
                None => String::from("ATTACKER"),
            };

            match data.invulnerables.get(ent) {
                None => {
                    corporeal.hp -= dmg;
                    data.message_log.log(format!("{} hits {} for {} damage!!", attacker_name, name.name, dmg));
                    if corporeal.hp <= 0 {
                        data.message_log.log(format!("{} is vanquished!!!", name.name));
                        match data.bodies.get(ent) {
                            Some(_) => {
                                corporeal.hp = corporeal.max_hp;
                                data.deaths.insert(ent, Death{});
                            },
                            None => {
                                data.entities.delete(ent);
                            }
                        }

                    }
                },

                Some(_) => {
                    data.message_log.log(String::from("It's as if the attack is deflected by a divine force!"));
                }
            }
        }
    }
}

pub struct DeathSystem;
impl<'a> System<'a> for DeathSystem {
    type SystemData = CombatSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (death, ent) in (&data.deaths, &data.entities).join() {
            data.corpses.insert(ent, Corpse{});
            data.world_updater.remove::<Death>(ent);
            if let Some(name) = data.names.get_mut(ent) {
                name.name = format!("corpse of {}", {&name.name});
            }

            data.actors.remove(ent);
            if let Some(renderable) = data.renderables.get_mut(ent) {
                renderable.fg_color = (100, 100, 100);
                renderable.bg_color = Some((60,0,0));
            }

            if let Some(pos) = data.positions.get(ent) {
                let mut view = data.view.map.lock().unwrap();
                data.entity_map.actors.set_point(pos.x, pos.y, None);
                view.set(pos.x, pos.y, true, true);
            }

            data.elevations.insert(ent, Elevation::OnFloor);

            data.move_requests.remove(ent);
            data.attack_requests.remove(ent);
            data.action_results.remove(ent);
            data.ai_units.remove(ent);

        }
    }
}