use crate::prelude::*;
const NUM_TILES: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WorldTileType {
    Town(LocalMapID),
    Dungeon(LocalMapID),
    Desert,
}

pub struct WorldMap {
    pub tiles: [WorldTileType; NUM_TILES],
    pub revealed_tiles: [bool; NUM_TILES],
}

impl WorldMap {
    pub fn new() -> Self {
        let mut tiles: [WorldTileType; NUM_TILES] = [WorldTileType::Desert; NUM_TILES];
        Self {
            tiles,
            revealed_tiles: [false; NUM_TILES],
        }
    }
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < MAP_WIDTH && point.y >= 0 && point.y < MAP_HEIGHT
    }
}
