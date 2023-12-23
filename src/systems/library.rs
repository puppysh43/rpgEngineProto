use crate::prelude::*;

///gets you all the necessary information to limit an AI system to the player's current
///map screen as a tuple.
pub fn get_player_info_and_map(state: &mut State) -> (LocalMapID, Point3D, Point, MapScreen) {
    let player = state
        .ecs
        .query_mut::<With<(&CurrentLocalMap, &Point3D, &Point), &Player>>(); //query containing all the needed player info
    let mut player_pos = Point::new(0, 0);
    let mut player_pos3d = Point3D::new(0, 0, 0);
    let mut player_localmap = LocalMapID::FirstTown; //temp filler location that will be overwritten
                                                     //iterate through the player query to get the relevant information. this generally safer and neater than using nth and unwrap
                                                     //use the query to get the player info
    for (_, (localmap, pos_3d, pos)) in player {
        player_localmap = localmap.0;
        player_pos3d = *pos_3d;
        player_pos = *pos;
    }

    let current_mapscreen = state
        .localmaps
        .get(player_localmap)
        .get_mapscreen(player_pos3d); //use player info to get the current mapscreen.
    (player_localmap, player_pos3d, player_pos, current_mapscreen)
}
