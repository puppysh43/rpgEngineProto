use crate::prelude::*;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
///Used both to keep track of where different map screens in a location are in relationship
///to each other as well as a component for ALL entities to track where in their location they are
pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3D {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

///ID Enum for identifying Locations
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LocationID {
    FirstTown,
    FirstDungeon,
}
///This is the unit that represents each discrete location on the worldmap that the player can go down and interact with
pub struct Location {
    maps: HashMap<Point3D, Map>, //all the different map screens in the location
    spawn_pos: Point,            //where the player is spawned when the first enter the location
}

impl Location {
    ///Method for initializing a new location, you give it a map and it will generate the hasmap as well as ensure that there is always a map
    ///w/ the coords 0,0,0 to ensure nothing breaks!
    pub fn new(first_map: Map, spawn_pos: Point) -> Self {
        let mut maps: HashMap<Point3D, Map> = HashMap::new();
        maps.insert(Point3D::new(0, 0, 0), first_map);
        Self { maps, spawn_pos }
    }
    ///Method for adding a new map to the location that ensures it will always be adjacent to another map in any of the 6
    ///3D cardinal directions.
    pub fn add_map(&self, map: Map, origin: Point3D, direction: CardinalDirection) {
        self.maps
            .get(&origin)
            .expect("There is not a valid map at the given origin location when adding a map.");
        let mut newpoint = origin;
        match direction {
            CardinalDirection::North => newpoint.y = newpoint.y + 1,
            CardinalDirection::East => newpoint.x = newpoint.x + 1,
            CardinalDirection::South => newpoint.y = newpoint.y - 1,
            CardinalDirection::West => newpoint.x = newpoint.x - 1,
            CardinalDirection::Up => newpoint.z = newpoint.z + 1,
            CardinalDirection::Down => newpoint.z = newpoint.z - 1,
        }
        self.maps.insert(newpoint, map);
    }
    ///Method for getting a map from the location given a 3D point
    pub fn get_map(&self, pos: Point3D) -> Map {
        let map = self
            .maps
            .get(&pos)
            .expect("could not get map, provided incorrect 3D coordinates.");
        *map
    }
    ///method for checking if there's a valid map at the given 3D point
    pub fn check_map(&self, pos: Point3D) -> bool {
        let is_map = self.maps.get(&pos).is_some();
        is_map
    }
}
