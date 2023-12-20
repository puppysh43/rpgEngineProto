use crate::prelude::*;

///gets you all the necessary information to limit an AI system to the player's current
///map screen as a tuple.
pub fn get_player_info_and_map(
    state: &mut State,
    commands: &mut CommandBuffer,
) -> (LocationID, Point3D, Point, Map) {
    let player = state
        .ecs
        .query_mut::<With<(&CurrentLocation, &Point3D, &Point), &Player>>(); //query containing all the needed player info
    let mut player_pos = Point::new(0, 0);
    let mut player_pos3d = Point3D::new(0, 0, 0);
    let mut player_location = LocationID::FirstTown; //temp filler location that will be overwritten
                                                     //iterate through the player query to get the relevant information. this generally safer and neater than using nth and unwrap
                                                     //use the query to get the player info
    for (_, (location, pos_3d, pos)) in player {
        player_location = location.0;
        player_pos3d = *pos_3d;
        player_pos = *pos;
    }

    let current_mapscreen = state.locations.get(player_location).get_map(player_pos3d); //use player info to get the current mapscreen.
    (player_location, player_pos3d, player_pos, current_mapscreen)
}
