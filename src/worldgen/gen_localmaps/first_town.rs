use crate::prelude::*;
use std::fs;

pub fn first_town() -> LocalMap {
    //read the raw string from a text file
    let north_raw = fs::read_to_string("resources/map_blueprints/first_town/north.txt")
        .expect("failed to read string from first town north sector text file.");
    //convert that string into a mapscreen file
    let north = MapScreen::from_string(north_raw);
    let center_raw = fs::read_to_string("resources/map_blueprints/first_town/center.txt")
        .expect("failed to read string for first town center");
    let center = MapScreen::from_string(center_raw);
    let east_raw = fs::read_to_string("resources/map_blueprints/first_town/east.txt")
        .expect("failed to read text file for first town east.");
    let east = MapScreen::from_string(east_raw);
    let south_raw = fs::read_to_string("resources/map_blueprints/first_town/south.txt")
        .expect("failed to read text file for first town south");
    let south = MapScreen::from_string(south_raw);
    let spawn_pos = Point::new(25, 0);
    let mut first_town = LocalMap::new(north, spawn_pos);
    first_town.add_mapscreen(center, Point3D::new(0, 0, 0), CardinalDirection::South);
    first_town.add_mapscreen(east, Point3D::new(0, -1, 0), CardinalDirection::East);
    first_town.add_mapscreen(south, Point3D::new(0, -1, 0), CardinalDirection::South);
    first_town.connect_mapscreens();

    first_town
}
