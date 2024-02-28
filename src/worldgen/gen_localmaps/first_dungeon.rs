//will later use private functions within this module to define different levels of the dungeon

use crate::prelude::*;
use crate::worldgen::mapdataraw::MapDataRaw;
use std::fs;

pub fn first_dungeon() -> LocalMap {
    let lvl_1_raw_json =
        fs::read_to_string("resources/map_blueprints/first_dungeon/dungeon_lvl_1.json")
            .expect("failed to read json from file");
    let lvl_2_raw_json =
        fs::read_to_string("resources/map_blueprints/first_dungeon/dungeon_lvl_2.json")
            .expect("failed to read json from file");
    let lvl_1_data: MapDataRaw = serde_json::from_str(&lvl_1_raw_json).unwrap();
    let lvl_2_data: MapDataRaw = serde_json::from_str(&lvl_2_raw_json).unwrap();
    let lvl_1 = lvl_1_data.export_mapscreen();
    let lvl_2 = lvl_2_data.export_mapscreen();
    let spawn_pos = Point::new(0, 0);
    let mut first_dungeon = LocalMap::new(lvl_1, spawn_pos);
    first_dungeon.add_mapscreen(lvl_2, Point3D::new(0, 0, 0), CardinalDirection::Down);
    first_dungeon
}
