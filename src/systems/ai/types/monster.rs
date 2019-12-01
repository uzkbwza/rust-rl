use super::super::pathfinding;
use super::super::thinking::Thinking;
use super::super::AiSystemData;
use crate::command::Command;
use crate::components::{CanSeeTarget, Target};
use crate::systems::movement::Dir;
use specs::prelude::*;
use tcod::map::FovAlgorithm;

pub struct Monster;

impl Thinking for Monster {
    fn get_command(entity: Entity, data: &AiSystemData) -> Vec<Command> {
        if Self::can_target_player(entity, data) {
            if let (Some(pos), Some(target)) =
                (data.positions.get(entity), data.targets.get(entity))
            {
                if let Some(target_pos) = data.positions.get(target.entity) {
                    let dx = target_pos.x - pos.x;
                    let dy = target_pos.y - pos.y;
                    // println!("{:?}", (dx, dy));

                    // check if adjacent
                    if i32::abs(dx) <= 1 && i32::abs(dy) <= 1 {
                        return vec![Command::Attack(Dir::pos_to_dir((dx, dy)))];
                    }

                    let mut path = Vec::new();

                    for step in pathfinding::path_to_target(entity, data) {
                        path.push(Command::Move(step));
                    }
                    return path;
                }
            }
        }
        return vec![Command::Move(Dir::Nowhere)];
    }
}

impl Monster {
    fn can_target_player(entity: Entity, data: &AiSystemData) -> bool {
        let mut fov_map = data.view.map.lock().unwrap();

        if let (Some(pos), Some(seer)) = (data.positions.get(entity), data.seers.get(entity)) {
            for (player_entity, target_pos, _player) in
                (&data.entities, &data.positions, &data.players).join()
            {
                fov_map.compute_fov(pos.x, pos.y, seer.fov, true, FovAlgorithm::Basic);
                let target_is_in_fov = fov_map.is_in_fov(target_pos.x, target_pos.y);

                if data.targets.get(entity) != None {
                    if data.sees_targets.get(entity) != None && !target_is_in_fov {
                        // println!("lost sight");
                        data.world_updater.remove::<CanSeeTarget>(entity)
                    }
                    return true;
                }

                if target_is_in_fov {
                    if data.sees_targets.get(entity) == None {
                        // println!("gained sight");
                        data.world_updater.insert(entity, CanSeeTarget {});
                        data.world_updater
                            .insert(entity, Target::new(player_entity, *target_pos));
                        return true;
                    }
                }
            }
        }
        false
    }
}
