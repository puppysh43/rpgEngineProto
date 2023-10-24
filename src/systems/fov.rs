use crate::prelude::*;

pub fn fov(state: &mut State, commands: &mut CommandBuffer) {
    let mut views = state.ecs.query::<(&Point, &mut FieldOfView)>();
    views //this needs to be commented better I'm still not sure how exactly
        .iter()
        .filter(|(_, (_, fov))| fov.is_dirty)
        .for_each(|(_, (pos, fov))| {
            fov.visible_tiles = field_of_view_set(*pos, fov.radius, &state.map);
            fov.is_dirty = false;
        });
}
