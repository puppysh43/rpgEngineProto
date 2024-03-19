use crate::prelude::*;
use crate::systems::library::*;

pub enum TimePhase {
    Dawn,
    Midday,
    Dusk,
    Night,
}

///This is the parent system that does all the calculations needed to
///keep the lightmap updated. May split off into sub-functions later
///as scale grows.
pub fn lighting_system(state: &mut State, commands: &mut CommandBuffer) {
    let (localmap_id, point3D, pos, mut current_mapscreen) = get_player_info_and_map(state);
    let current_timephase = current_timephase(state);
    if point3D.z < 0 {
        apply_timephase(&mut current_mapscreen, current_timephase);
    }
    apply_light_sources(state, &mut current_mapscreen);

    state
        .localmaps
        .update_mapscreen(localmap_id, point3D, current_mapscreen);
}

fn apply_timephase(mapscreen: &mut MapScreen, time_phase: TimePhase) {
    //first step of updating the lightmap is translating the number of turns into a time
    match time_phase {
        TimePhase::Dawn => {
            mapscreen.light_map = [1; NUM_TILES];
        }
        TimePhase::Midday => {
            mapscreen.light_map = [2; NUM_TILES];
        }
        TimePhase::Dusk => {
            mapscreen.light_map = [1; NUM_TILES];
        }
        TimePhase::Night => {
            mapscreen.light_map = [0; NUM_TILES];
        }
    }
}

fn apply_light_sources(state: &mut State, mapscreen: &mut MapScreen) {
    let (localmap_id, player_pos3d, pos, _) = get_player_info_and_map(state);
    let mut light_sources = state
        .ecs
        .query::<(&CurrentLocalMap, &Point3D, &Point, &mut LightSource)>();
    for (_, (current_localmap, pos_3d, pos, light_source)) in light_sources.iter() {
        if current_localmap.0 == localmap_id && *pos_3d == player_pos3d {
            //update the light sources in case you have moving lights
            if light_source.is_dirty {
                light_source.lit_tiles = field_of_view_set(*pos, light_source.radius, mapscreen);
                light_source.is_dirty = false;
            }
            //add one additional light level to tiles effected by a light source
            for pos in &light_source.lit_tiles {
                mapscreen.light_map[map_idx(pos.x, pos.y)] += 1;
            }
        }
    }
}
