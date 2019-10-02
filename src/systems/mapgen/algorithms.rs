use array2d::Array2D;
use rand::prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum TileType {
    Floor,
    Wall,
    _Window,
    _Door
}

pub fn random(width: usize, height: usize) -> Array2D<TileType> {
    let mut map = Array2D::filled_with(TileType::Wall, width, height);
    let mut rng = thread_rng();
    let tile_types = [
        TileType::Floor, 
        TileType::Wall
    ];

    for x in 0..width {
        for y in 0..height {
            map[(x, y)] = *tile_types.choose(&mut rng).unwrap();
        }
    }
    map
}

