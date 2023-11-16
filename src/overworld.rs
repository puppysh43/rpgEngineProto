use crate::prelude::*;
const NUM_TILES: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WorldTileType {
    Town(LocationID),
    Dungeon(LocationID),
    Desert,
}

pub struct WorldMap {
    pub tiles: Vec<WorldTileType>,
    pub revealed_tiles: Vec<bool>,
}

impl WorldMap {
    pub fn new() -> Self {
        let mut tiles: Vec<WorldTileType> = vec![WorldTileType::Desert; NUM_TILES];
        Self {
            tiles,
            revealed_tiles: vec![false; NUM_TILES],
        }
    }
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < MAP_WIDTH && point.y >= 0 && point.y < MAP_HEIGHT
    }
}
