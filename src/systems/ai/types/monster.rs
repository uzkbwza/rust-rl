use specs::prelude::*;
use tcod::map::FovAlgorithm;
use crate::command::Command;
use crate::components::{Target, CanSeeTarget};
use crate::systems::movement::{Dir};
use super::super::thinking::Thinking;
use super::super::AiSystemData;
use super::super::pathfinding;

pub struct Monster;

impl Thinking for Monster {
    fn get_command(entity: Entity, data: &AiSystemData) -> Option<Command> {
        if Self::can_target_player(entity, data) {
            return Some(Command::Move(pathfinding::path_to_target(entity, data)))
        }
        return Some(Command::Move(Dir::Nowhere))
    }
}

impl Monster {
    fn can_target_player(entity: Entity, data: &AiSystemData) -> bool {
        let mut fov_map = data.view.map.lock().unwrap();

        if let (Some(pos), Some(seer)) = (data.positions.get(entity), data.seers.get(entity)) {
            for (player_entity, target_pos, _player) in (&data.entities, &data.positions, &data.players).join() {
                fov_map.compute_fov(pos.x, pos.y, seer.fov, true, FovAlgorithm::Basic);
                let target_is_in_fov = fov_map.is_in_fov(target_pos.x, target_pos.y);

                if data.targets.get(entity) != None {
                    if data.sees_targets.get(entity) != None && !target_is_in_fov {
                        // println!("lost sight");
                        data.world_updater.remove::<CanSeeTarget>(entity)
                    }
                    return true
                }

                if target_is_in_fov {
                    if data.sees_targets.get(entity) == None {
                        // println!("gained sight");
                        data.world_updater.insert(entity, CanSeeTarget{});
                        data.world_updater.insert(entity, Target::new(player_entity));
                        return true
                    }
                }
            }
        }
        false
    }
}

