use crate::prelude::*;
mod first_town;
use first_town::*;
mod first_dungeon;
use first_dungeon::*;
pub const NUM_LOCALMAPS: usize = 2;
///This function generates all of the locations in the game that the player can visit
///both static locations as well as things like random encounters. Edit this when building
///your game out of this engine!
pub fn generate_localmaps() -> LocalMaps {
    let localmaps = LocalMaps::new([first_town(), first_dungeon()]);
    localmaps
}
