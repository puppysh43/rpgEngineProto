use crate::prelude::*;

pub fn build_devroom01() -> Map {
    let devroom = Map::blank_canvas(TileType::Floor);
    let portal_pos = map_idx(5, 5);
    devroom.tiles[portal_pos] = TileType::MapPortal {
        destination: MapID::DevRoom02,
    };
    devroom
}
pub fn build_devroom02() -> Map {}
