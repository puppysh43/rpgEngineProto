use crate::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileType {
    Wall,
    Floor,
    StairUp,
    StairDown,
    // Elevator,
    MapTransitionNorth,
    MapTransitionEast,
    MapTransitionSouth,
    MapTransitionWest,
    //ThinWall,
    //ThickWall,
}

///Utility function that gives you an index for accessing the mapscreen given a 2D Point
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * MAP_WIDTH) + x) as usize
}

#[derive(Clone, Debug)]
///Smallest unit of the the worldspace. Holds all the information you need to actually render the gameworld,
///from the tiles of the map to lighting or FOV information.
pub struct MapScreen {
    ///Array of tiletypes with a predefined size
    pub tiles: [TileType; NUM_TILES],
    ///Array of what tiles have fallen within the player's FOV ever
    pub revealed_tiles: [bool; NUM_TILES],
    ///Array of the light level of each worldtile. Dynamically changed by time of day, light sources, etc
    pub light_map: [i32; NUM_TILES],
    //pub default_lightlevel: Option<i32>
}

impl MapScreen {
    ///Generate a new mapscreen composed entirely of a floor.
    pub fn new() -> Self {
        Self {
            tiles: [TileType::Floor; NUM_TILES],
            revealed_tiles: [false; NUM_TILES],
            light_map: [0; NUM_TILES],
        }
    }

    ///Generate a new mapscreen composed entirely of one tiletype
    pub fn blank_canvas(tile: TileType) -> Self {
        Self {
            tiles: [tile; NUM_TILES],
            revealed_tiles: [false; NUM_TILES],
            light_map: [0; NUM_TILES],
        }
    }
    ///create a new map file from a given string
    pub fn from_string(mut raw_string: String) -> Self {
        //this will take in a string and convert it into a map, or return a blank map of only floors if it's not the right size or otherwise not fit for purpose
        let mut map: [TileType; NUM_TILES] = [TileType::Floor; NUM_TILES];

        raw_string.retain(|c| !c.is_whitespace());
        //need to trim the string and make sure it's exactly NUM_TILES long
        if raw_string.len() == NUM_TILES {
            let mut i: usize = 0;
            for char in raw_string.chars() {
                match char {
                    '#' => map[i] = TileType::Wall,
                    '.' => map[i] = TileType::Floor,
                    '<' => map[i] = TileType::StairUp,
                    '>' => map[i] = TileType::StairDown,
                    _ => map[i] = TileType::Floor,
                }
                i += 1;
            }

            return Self {
                tiles: map,
                revealed_tiles: [false; NUM_TILES],
                light_map: [0; NUM_TILES],
            };
        } else {
            return Self {
                tiles: [TileType::Floor; NUM_TILES],
                revealed_tiles: [false; NUM_TILES],
                light_map: [0; NUM_TILES],
            };
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < MAP_WIDTH && point.y >= 0 && point.y < MAP_HEIGHT
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        // self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor

        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] != TileType::Wall
    }
    ///function used to see if a position on the map is a valid location
    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            //make sure the destination is even in the mapscreen array to avoid crashes
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Algorithm2D for MapScreen {
    fn dimensions(&self) -> Point {
        Point::new(MAP_WIDTH, MAP_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point)
    }
}

impl BaseMap for MapScreen {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] != TileType::Floor
    }
    ///calculates the weight of varying tiles given certain conditions. Tweak this to change the pathfinding
    ///algorithm's preference (ex. to avoid traps, how often they move diagonally, or to stick to roads)
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new(); //uses a smallvec for optimization b/c there will only ever be a small amount of
        let location = self.index_to_point2d(idx); //the location the moving entity wants to go to

        //this logic controls the weights of various possible exits
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}
