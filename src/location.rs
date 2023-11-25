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

#[derive(Clone, Debug)]
///This is the unit that represents each discrete location on the worldmap that the player can go down and interact with
pub struct Location {
    maps: HashMap<Point3D, Map>, //all the different map screens in the location
    spawn_pos: Point,            //where the player is spawned when the first enter the location
                                 // has_been_entered: bool, //used to keep track of if it's been entered by the player before. maybe
                                 // description: String, //description of the location printed to the log every time it's entered.
                                 //pallete variable that controls what the color of stuff like the walls and floor.
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
    pub fn add_map(&mut self, map: Map, origin: Point3D, direction: CardinalDirection) {
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
            .expect("could not get map, provided incorrect 3D coordinates.")
            .clone();
        map
    }
    ///method for checking if there's a valid map at the given 3D point
    pub fn check_map(&self, pos: Point3D) -> bool {
        let is_map = self.maps.get(&pos).is_some();
        is_map
    }
    ///method that returns the spawnpoint
    pub fn get_spawnpos(&self) -> Point {
        self.spawn_pos
    }
    ///only used when a map needs to be updated
    pub fn update_map(&mut self, pos_3d: Point3D, map: Map) {
        self.maps.insert(pos_3d, map);
    }
    ///This function looks at all the maps in the location and places map transition blocks at the edges
    ///where appropriate. Run after inserting all the maps into the location.
    pub fn connect_maps(&mut self) {
        let maps_ref = self.clone(); //this will be used to check for adjacency and stuff while iterating through the actual maps and editing them.
        for (pos_3d, map) in self.maps.iter_mut() {
            //iterate through the entire location
            //define 3D points adjacent to the current map in the iterator
            let adjacent_north = Point3D::new(pos_3d.x, pos_3d.y + 1, pos_3d.z);
            let adjacent_east = Point3D::new(pos_3d.x + 1, pos_3d.y, pos_3d.z);
            let adjacent_south = Point3D::new(pos_3d.x, pos_3d.y - 1, pos_3d.z);
            let adjacent_west = Point3D::new(pos_3d.x - 1, pos_3d.y, pos_3d.z);
            //all of the following code checks for adjacent maps if there are adjacent maps runs through
            //the current map and the adjacent map checking for tiles where there's floor tiles "next"
            //to each other on the virtual edges and changing those to a map transition tile
            if maps_ref.check_map(adjacent_north) {
                let adj_map = maps_ref.get_map(adjacent_north);

                for x in 0..MAP_WIDTH {
                    if map.tiles[map_idx(x, 0)] != TileType::Wall
                        && adj_map.tiles[map_idx(x, MAP_HEIGHT - 1)] != TileType::Wall
                    {
                        map.tiles[map_idx(x, 0)] = TileType::MapTransitionNorth;
                    }
                }
            }
            if maps_ref.check_map(adjacent_east) {
                let adj_map = maps_ref.get_map(adjacent_east);

                for y in 0..MAP_HEIGHT {
                    if map.tiles[map_idx(MAP_WIDTH - 1, y)] != TileType::Wall
                        && adj_map.tiles[map_idx(0, y)] != TileType::Wall
                    {
                        map.tiles[map_idx(MAP_WIDTH - 1, y)] = TileType::MapTransitionEast;
                    }
                }
            }
            if maps_ref.check_map(adjacent_south) {
                let adj_map = maps_ref.get_map(adjacent_south);

                for x in 0..MAP_WIDTH {
                    if map.tiles[map_idx(x, MAP_HEIGHT - 1)] != TileType::Wall
                        && adj_map.tiles[map_idx(x, 0)] != TileType::Wall
                    {
                        map.tiles[map_idx(x, MAP_HEIGHT - 1)] = TileType::MapTransitionSouth;
                    }
                }
            }
            if maps_ref.check_map(adjacent_west) {
                let adj_map = maps_ref.get_map(adjacent_west);

                for y in 0..MAP_HEIGHT {
                    if map.tiles[map_idx(0, y)] != TileType::Wall
                        && adj_map.tiles[map_idx(MAP_WIDTH - 1, y)] != TileType::Wall
                    {
                        map.tiles[map_idx(0, y)] = TileType::MapTransitionWest;
                    }
                }
            }
        }
    }
}

pub struct Locations([Location; NUM_LOCATIONS]);

impl Locations {
    pub fn new(contents: [Location; NUM_LOCATIONS]) -> Self {
        Self(contents)
    }
    pub fn get(&self, location_id: LocationID) -> &Location {
        &self.0[location_id as usize]
    }

    pub fn get_mut(&mut self, location_id: LocationID) -> &mut Location {
        &mut self.0[location_id as usize]
    }
    pub fn update(&mut self, location_id: LocationID, location: Location) {
        self.0[location_id as usize] = location;
    }
}
