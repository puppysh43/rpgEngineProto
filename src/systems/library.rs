use crate::prelude::*;

use super::lighting_system::TimePhase;

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
///Helper function that looks for whatever interaction menu has the active tag assigned to its respective
///key in the ECS and then returns it, using an option wrapper for safety
pub fn get_active_interactionmenu(state: &State) -> Option<InteractionMenu> {
    let mut interaction_menu_key = String::new(); //make a blank string to hold the key after you query for it
    let mut int_menu_query = state
        .ecs
        .query::<With<&InteractionMenuKey, &ActiveInteractionMenu>>();
    //query the key of the interaction menu
    for (_, int_menu_key) in int_menu_query.iter() {
        interaction_menu_key = int_menu_key.0.clone();
    }
    //using the active key you got from the ecs retrieve a copy of the active interaction menu from the database
    //in the state
    let interaction_menu = state.int_menu_db.get_interaction_menu(interaction_menu_key);
    interaction_menu
}

pub fn get_bg_color(state: &State) -> (u8, u8, u8) {
    let mut num_turns = state.numberturns;
    if num_turns > 17280 {
        num_turns = num_turns % 17280;
    }
    if num_turns < 4320 {
        DYN_BG.dark
    } else if num_turns < 7200 {
        DYN_BG.dim
    } else if num_turns < 11520 {
        DYN_BG.bright
    } else if num_turns < 14400 {
        DYN_BG.dim
    } else {
        DYN_BG.dark
    }
}

pub fn current_timephase(state: &State) -> TimePhase {
    //each turn is 5 seconds
    //each minute is 12 turns
    //each hour is 720 turns
    //each day is 17280 turns
    let mut num_turns = state.numberturns;
    if num_turns > 17280 {
        num_turns = num_turns % 17280;
    }
    if num_turns < 4320 {
        TimePhase::Night
    } else if num_turns < 7200 {
        TimePhase::Dawn
    } else if num_turns < 11520 {
        TimePhase::Midday
    } else if num_turns < 14400 {
        TimePhase::Dusk
    } else {
        TimePhase::Night
    }
}
