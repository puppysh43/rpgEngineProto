use crate::prelude::*;

use super::library::get_player_info_and_map;

pub fn fov(state: &mut State, commands: &mut CommandBuffer) {
    let (player_localmap_id, player_pos3d, player_pos, map) = get_player_info_and_map(state);
    let mut views = state
        .ecs
        .query::<(&CurrentLocalMap, &Point3D, &Point, &mut FieldOfView)>();

    views //this needs to be commented better I'm still not sure how exactly this shit works
        .iter()
        .filter(|(_, (current_localmap, pos_3d, _, _))| {
            current_localmap.0 == player_localmap_id && **pos_3d == player_pos3d
        })
        .filter(|(_, (_, _, _, fov))| fov.is_dirty)
        .for_each(|(_, (_, _, pos, fov))| {
            fov.visible_tiles = field_of_view_set(*pos, fov.radius, &map);
            fov.is_dirty = false;
        });
}
