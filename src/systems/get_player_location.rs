use crate::prelude::*;

pub fn get_player_location(state: &mut State) {
    let mut player_location = MapID::DevRoom01; //filler that will be immediately overwritten but we can't have a null variable
    for (_, loc) in state.ecs.query::<With<&Location, &Player>>().iter() {
        player_location = loc.0; //get the player's location and store it
    }
    state.player_location = player_location; //stick it in that gamestate struct for every other system to use

    //query for the player's location and update the info in the struct
}
