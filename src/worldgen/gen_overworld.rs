use crate::prelude::*;
///Function that generates the overworld map. This is what you will be fiddling with to
///customize the overworld for your game!
pub fn generate_overworld() -> WorldMap {
    let overworld = WorldMap::new();
    let first_town_pos = Point::new(3, 4);
    let first_dungeon = Point::new(10, 10);

    overworld.tiles[map_idx(first_town_pos.x, first_town_pos.y)] =
        WorldTileType::Town(LocationID::FirstTown);
    overworld.tiles[map_idx(first_dungeon.x, first_dungeon.y)] =
        WorldTileType::Dungeon(LocationID::FirstDungeon);

    overworld
}
