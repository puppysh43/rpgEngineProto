use crate::prelude::*;

pub fn map_transitions(state: &mut State) {
    let (player_entity, player_location) = state
        .ecs
        .query::<With<&Location, &Player>>()
        .iter()
        .nth(0)
        .expect("There isn't a player.");
    let player_mapid: MapID = Some(); //filler or something?
                                      //god this code is so fucking messy
    let current_map = state.localmaps.get(&player_location.0).unwrap();
    let mut player_pos = Point::new(0, 0);
    for (_, pos) in state.ecs.query::<With<&Point, &Player>>().iter() {
        player_pos = *pos;
    }
    let player_idx = map_idx(player_pos.x, player_pos.y);

    let player_tile = current_map.tiles[player_idx];
    //need to make this generic so it doesn't care what the destination is
    //it just extracts the destination
    //will check if the player is in the same tile as a map portal, will get the mapID from that portal
    //and then switch the location of the player to that location.
}
