use crate::prelude::*;
///Function that generates the worldmap. This is what you will be fiddling with to
///customize the worldmap for your game!
pub fn generate_worldmap() -> WorldMap {
    let mut worldmap = WorldMap::new();
    let first_town_pos = Point::new(3, 4);
    let first_dungeon = Point::new(10, 10);

    worldmap.tiles[map_idx(first_town_pos.x, first_town_pos.y)] =
        WorldTileType::Town(LocalMapID::FirstTown);
    worldmap.tiles[map_idx(first_dungeon.x, first_dungeon.y)] =
        WorldTileType::Dungeon(LocalMapID::FirstDungeon);

    worldmap
}
