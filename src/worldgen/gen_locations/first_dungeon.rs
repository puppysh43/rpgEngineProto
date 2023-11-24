//will later use private functions within this module to define different levels of the dungeon

use crate::prelude::*;
use std::fs;

pub fn first_dungeon() -> Location {
    //will need to change this later
    let lvl_1_raw = fs::read_to_string("resources/map_blueprints/first_dungeon/lvl_1.txt")
        .expect("Failed to read string from text file.");
    let lvl_1 = Map::from_string(lvl_1_raw);
    let lvl_2_raw = fs::read_to_string("resources/map_blueprints/first_dungeon/lvl_2.txt")
        .expect("failed to read string from level 2 text file.");
    let lvl_2 = Map::from_string(lvl_2_raw);
    let spawn_pos = Point::new(0, 0);
    let mut first_dungeon = Location::new(lvl_1, spawn_pos);
    first_dungeon.add_map(lvl_2, Point3D::new(0, 0, 0), CardinalDirection::Down);
    first_dungeon
}
