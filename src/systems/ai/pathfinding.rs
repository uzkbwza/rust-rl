use specs::prelude::*;
use tcod::map::FovAlgorithm;
use tcod::map::Map as TcodMap;
use crate::systems::movement::{Dir};
use crate::map::EntityMap;
use tcod::pathfinding::Dijkstra;
use super::AiSystemData;

pub fn rank_distance(dest: (i32, i32), point: (i32, i32), fov_map: &TcodMap, pathfinder: &mut Dijkstra, entity_map: &EntityMap) -> Option<(i32, (i32, i32))> {
    let x = point.0;
    let y = point.1;
    pathfinder.find((x, y));
    // println!("{}", num_steps);
    let ranking = (((dest.0 - x).pow(2) + (dest.1 - y).pow(2)) as f32).sqrt() as i32 ;
    if !fov_map.is_walkable(x as i32, y as i32) {
        return None
    }

    if let Ok(point) = entity_map.actors.retrieve(x as i32, y as i32) {
        if let Some(actor) = point {
            return None
        }
    }
    Some((ranking, (x, y)))
}

pub fn choose_close_point(range: i32, start: (i32, i32), dest: (i32, i32), fov_map: &TcodMap, pathfinder: &mut Dijkstra, entity_map: &EntityMap) -> (i32, i32) {
    let mut rankings = Vec::new();
    for x in (start.0 - range)..(start.0 + range) + 1  {
        for y in (start.1 - range)..(start.1 + range) + 1 {
            if x <= 0 ||
                y <= 0 || x >= entity_map.width as i32 || y >= entity_map.height as i32 {
                continue
            }
            if let Some(ranking) = rank_distance(dest, (x, y), fov_map, pathfinder, entity_map) {
                rankings.push(ranking);
            }
        }
    }
    if rankings.is_empty() { return start }
    rankings.sort_by(|a, b| a.0.cmp(&b.0));
    
    let closest = ((rankings[0].1).0 as i32, (rankings[0].1).1 as i32);
    closest
}


pub fn path_to_target(entity: Entity, data: &AiSystemData) -> Vec<Dir> {
    if let (Some(target), Some(pos), Some(seer)) = (data.targets.get(entity), data.positions.get(entity), data.seers.get(entity)) {
        if let Some(dest) = data.positions.get(target.entity) {
            
            let mut fov_map = data.view.map.lock().unwrap();

            fov_map.compute_fov(pos.x, pos.y, seer.fov, true, FovAlgorithm::Basic);
            let mut step_pos = (pos.x, pos.y);

            let mut pathfinder = Dijkstra::new_from_map(fov_map.clone(), f32::sqrt(2.0));
            pathfinder.compute_grid(step_pos);
            let dest_point = choose_close_point(3, (pos.x, pos.y), (dest.x, dest.y), &fov_map, &mut pathfinder, &data.entity_map);

            if pathfinder.find((dest_point.0, dest_point.1)) {
                if let Some(_) = pathfinder.get(0) {
                    let mut path = Vec::new();
                    for step in pathfinder.iter() {
                        let dx = step.0 - pos.x;
                        let dy = step.1 - pos.y;
                        let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();
                        let dx = (dx as f32 / distance).round() as i32;
                        let dy = (dy as f32 / distance).round() as i32;
                        if pos.x == step.0 && pos.y == step.1 {
                            path.push(Dir::Nowhere);
                            return path
                        }
                        path.insert( 0, Dir::pos_to_dir((dx, dy)));
                    }
                    return path
                }
            } else {
                return vec![Dir::Nowhere];
            }
        }
    }
    vec![Dir::Nowhere]
}
