use crate::prelude::*;

///ID Enum for identifying Locations
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LocationID {
    FirstTown,
    FirstDungeon,
}

pub struct Location {
    // maps: HashMap<Point3, Map>,
    maps: Vec<(Point3, Map)>,
}
//consider newtyping maps tuple vec with methods for accessing it to increase safety
