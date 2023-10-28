use crate::prelude::*;

const NUM_TILES: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;
#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
pub enum MapID {
    DevRoom01,
    DevRoom02,
    DevRoom03,
    DevRoom04,
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileType {
    Wall,
    Floor,
    MapPortal(MapID),
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * MAP_WIDTH) + x) as usize
}

pub struct Structure {
    pub body: String,
    pub height: i32,
    pub width: i32,
}
impl Structure {
    pub fn new(bdy: String, hght: i32, wdth: i32) -> Self {
        Self {
            body: bdy,
            height: hght,
            width: wdth,
        }
    }
}

#[derive(Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed_tiles: vec![false; NUM_TILES],
        }
    }

    pub fn blank_canvas(tile: TileType) -> Self {
        Self {
            tiles: vec![tile; NUM_TILES],
            revealed_tiles: vec![false; NUM_TILES],
        }
    }

    pub fn new_from_string(raw_string: String) -> Self {
        //this will take in a string and convert it into a map, or return a blank map of only floors if it's not the right size or otherwise not fit for purpose
        let mut map: Vec<TileType> = Vec::new();
        //need to trim the string and make sure it's exactly NUM_TILES long
        for i in raw_string.chars() {
            match i {
                '#' => map.push(TileType::Wall),
                '.' => map.push(TileType::Floor),
                _ => map.push(TileType::Floor),
            }
        }
        if map.len() == NUM_TILES {
            return Self {
                tiles: map,
                revealed_tiles: vec![false; NUM_TILES],
            };
        } else {
            return Self {
                tiles: vec![TileType::Floor; NUM_TILES],
                revealed_tiles: vec![false; NUM_TILES],
            };
        }
    }
    ///will insert tiles into a map, replacing them with the structure passed in given a point representing the location of the top left tile of the structure
    pub fn add(&mut self, structure: Structure, spawn_pos: Point) {
        //will need to add a function that somehow inserts the string into the vec at the right point.
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
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
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

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(MAP_WIDTH, MAP_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] != TileType::Floor
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

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
