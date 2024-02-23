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
pub enum LocalMapID {
    FirstTown,
    FirstDungeon,
}

#[derive(Clone, Debug)]
///This is the unit that represents each discrete location on the worldmap that the player can go down and interact with
pub struct LocalMap {
    mapscreens: HashMap<Point3D, MapScreen>, //all the different map screens in the location
    spawn_pos: Point, //where the player is spawned when the first enter the location
                      // has_been_entered: bool, //used to keep track of if it's been entered by the player before. maybe
                      // description: String, //description of the location printed to the log every time it's entered.
                      //pallete variable that controls what the color of stuff like the walls and floor.
}

impl LocalMap {
    ///Method for initializing a new location, you give it a map and it will generate the hasmap as well as ensure that there is always a map
    ///w/ the coords 0,0,0 to ensure nothing breaks!
    pub fn new(first_mapscreen: MapScreen, spawn_pos: Point) -> Self {
        let mut mapscreens: HashMap<Point3D, MapScreen> = HashMap::new();
        mapscreens.insert(Point3D::new(0, 0, 0), first_mapscreen);
        Self {
            mapscreens,
            spawn_pos,
        }
    }
    ///Method for adding a new map to the location that ensures it will always be adjacent to another map in any of the 6
    ///3D cardinal directions.
    pub fn add_mapscreen(
        &mut self,
        mapscreen: MapScreen,
        origin: Point3D,
        direction: CardinalDirection,
    ) {
        self.mapscreens
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
        self.mapscreens.insert(newpoint, mapscreen);
    }
    ///Method for getting a map from the location given a 3D point
    pub fn get_mapscreen(&self, pos: Point3D) -> MapScreen {
        let mapscreen = self
            .mapscreens
            .get(&pos)
            .expect("could not get map, provided incorrect 3D coordinates.")
            .clone();
        mapscreen
    }
    ///method for checking if there's a valid map at the given 3D point
    pub fn check_mapscreen(&self, pos: Point3D) -> bool {
        let is_mapscreen = self.mapscreens.get(&pos).is_some();
        is_mapscreen
    }
    ///method that returns the spawnpoint
    pub fn get_spawnpos(&self) -> Point {
        self.spawn_pos
    }
    ///only used when a map needs to be updated
    pub fn update_mapscreen(&mut self, pos_3d: Point3D, mapscreen: MapScreen) {
        self.mapscreens.insert(pos_3d, mapscreen);
    }
    ///This function looks at all the maps in the location and places map transition blocks at the edges
    ///where appropriate. Run after inserting all the maps into the location.
    pub fn connect_mapscreens(&mut self) {
        let mapscreens_ref = self.clone(); //this will be used to check for adjacency and stuff while iterating through the actual maps and editing them.
        for (pos_3d, mapscreen) in self.mapscreens.iter_mut() {
            //iterate through the entire location
            //define 3D points adjacent to the current map in the iterator
            let adjacent_north = Point3D::new(pos_3d.x, pos_3d.y + 1, pos_3d.z);
            let adjacent_east = Point3D::new(pos_3d.x + 1, pos_3d.y, pos_3d.z);
            let adjacent_south = Point3D::new(pos_3d.x, pos_3d.y - 1, pos_3d.z);
            let adjacent_west = Point3D::new(pos_3d.x - 1, pos_3d.y, pos_3d.z);
            //all of the following code checks for adjacent mapscreens if there are adjacent maps runs through
            //the current map and the adjacent map checking for tiles where there's floor tiles "next"
            //to each other on the virtual edges and changing those to a map transition tile
            if mapscreens_ref.check_mapscreen(adjacent_north) {
                let adj_mapscreen = mapscreens_ref.get_mapscreen(adjacent_north);

                for x in 0..MAP_WIDTH {
                    if mapscreen.tiles[map_idx(x, 0)] != TileType::Wall
                        && adj_mapscreen.tiles[map_idx(x, MAP_HEIGHT - 1)] != TileType::Wall
                    {
                        mapscreen.tiles[map_idx(x, 0)] = TileType::MapTransitionNorth;
                    }
                }
            }
            if mapscreens_ref.check_mapscreen(adjacent_east) {
                let adj_mapscreen = mapscreens_ref.get_mapscreen(adjacent_east);

                for y in 0..MAP_HEIGHT {
                    if mapscreen.tiles[map_idx(MAP_WIDTH - 1, y)] != TileType::Wall
                        && adj_mapscreen.tiles[map_idx(0, y)] != TileType::Wall
                    {
                        mapscreen.tiles[map_idx(MAP_WIDTH - 1, y)] = TileType::MapTransitionEast;
                    }
                }
            }
            if mapscreens_ref.check_mapscreen(adjacent_south) {
                let adj_mapscreen = mapscreens_ref.get_mapscreen(adjacent_south);

                for x in 0..MAP_WIDTH {
                    if mapscreen.tiles[map_idx(x, MAP_HEIGHT - 1)] != TileType::Wall
                        && adj_mapscreen.tiles[map_idx(x, 0)] != TileType::Wall
                    {
                        mapscreen.tiles[map_idx(x, MAP_HEIGHT - 1)] = TileType::MapTransitionSouth;
                    }
                }
            }
            if mapscreens_ref.check_mapscreen(adjacent_west) {
                let adj_mapscreen = mapscreens_ref.get_mapscreen(adjacent_west);

                for y in 0..MAP_HEIGHT {
                    if mapscreen.tiles[map_idx(0, y)] != TileType::Wall
                        && adj_mapscreen.tiles[map_idx(MAP_WIDTH - 1, y)] != TileType::Wall
                    {
                        mapscreen.tiles[map_idx(0, y)] = TileType::MapTransitionWest;
                    }
                }
            }
        }
    }
}

pub struct LocalMaps([LocalMap; NUM_LOCALMAPS]);

impl LocalMaps {
    pub fn new(contents: [LocalMap; NUM_LOCALMAPS]) -> Self {
        Self(contents)
    }
    pub fn get(&self, location_id: LocalMapID) -> &LocalMap {
        &self.0[location_id as usize]
    }

    pub fn get_mut(&mut self, location_id: LocalMapID) -> &mut LocalMap {
        &mut self.0[location_id as usize]
    }
    pub fn update(&mut self, location_id: LocalMapID, location: LocalMap) {
        self.0[location_id as usize] = location;
    }
}


//this block will be for the actual 
