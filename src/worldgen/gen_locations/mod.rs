use crate::prelude::*;
mod first_town;
use first_town::*;
///This function generates all of the locations in the game that the player can visit
///both static locations as well as things like random encounters. Edit this when building
///your game out of this engine!
pub fn generate_locations() -> HashMap<LocationID, Location> {
    let mut locations: HashMap<LocationID, Location> = HashMap::new();
    locations.insert(LocationID::FirstTown, first_town());
    locations
}
