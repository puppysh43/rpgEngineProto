use crate::prelude::*;
const NUM_TILES: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WorldTileType {
    Town(MapID),
    Dungeon(MapID),
    Desert,
}

pub struct WorldMap {
    pub tiles: Vec<WorldTileType>,
    pub revealed_tiles: Vec<bool>,
}

impl WorldMap {
    pub fn new() -> Self {
        let first_town_pos = Point::new(3, 4);
        let first_dungeon = Point::new(10, 10);
        let mut tiles: Vec<WorldTileType> = vec![WorldTileType::Desert; NUM_TILES];
        tiles[map_idx(first_town_pos.x, first_town_pos.y)] =
            WorldTileType::Town(MapID::FirstTownCenter);
        tiles[map_idx(first_dungeon.x, first_dungeon.y)] =
            WorldTileType::Dungeon(MapID::FirstDungeonLevel1);
        Self {
            tiles,
            revealed_tiles: vec![false; NUM_TILES],
        }
    }
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < MAP_WIDTH && point.y >= 0 && point.y < MAP_HEIGHT
    }
}
