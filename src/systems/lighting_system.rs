use crate::prelude::*;

enum TimePhase {
    Dawn,
    Midday,
    Dusk,
    Night,
}

use super::library::get_player_info_and_map;
///This is the parent system that does all the calculations needed to
///keep the lightmap updated. May split off into sub-functions later
///as scale grows.
pub fn lighting_system(state: &mut State, commands: &mut CommandBuffer) {
    let mut num_turns = state.numberturns;
    let (localmap_id, point3D, pos, mut current_mapscreen) = get_player_info_and_map(state);
    //first step of updating the lightmap is translating the number of turns into a time
    //each turn is 5 seconds
    //each minute is 12 turns
    //each hour is 720 turns
    //each day is 17280 turns
    if num_turns > 17280 {
        num_turns = num_turns % 17280;
    }
    let mut time_phase = TimePhase::Dawn;
    if num_turns < 4320 {
        time_phase = TimePhase::Night;
    } else if num_turns < 7200 {
        time_phase = TimePhase::Dawn;
    } else if num_turns < 11520 {
        time_phase = TimePhase::Midday;
    } else if num_turns < 14400 {
        time_phase = TimePhase::Dusk;
    } else {
        time_phase = TimePhase::Night;
    }
    match time_phase {
        TimePhase::Dawn => {
            current_mapscreen.light_map = [1; NUM_TILES];
        }
        TimePhase::Midday => {
            current_mapscreen.light_map = [2; NUM_TILES];
        }
        TimePhase::Dusk => {
            current_mapscreen.light_map = [1; NUM_TILES];
        }
        TimePhase::Night => {
            current_mapscreen.light_map = [0; NUM_TILES];
        }
    }
    //need to make update mapscreen method
    state
        .localmaps
        .update_mapscreen(localmap_id, point3D, current_mapscreen);
}
