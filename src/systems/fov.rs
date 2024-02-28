use crate::prelude::*;

use super::library::get_player_info_and_map;

pub fn fov(state: &mut State, commands: &mut CommandBuffer) {
    let (_, _, _, map) = get_player_info_and_map(state);
    let mut views = state.ecs.query::<(&Point, &mut FieldOfView)>();

    views //this needs to be commented better I'm still not sure how exactly this shit works
        .iter()
        .filter(|(_, (_, fov))| fov.is_dirty)
        .for_each(|(_, (pos, fov))| {
            fov.visible_tiles = field_of_view_set(*pos, fov.radius, &map);
            fov.is_dirty = false;
        });
}
