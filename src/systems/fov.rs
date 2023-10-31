use crate::prelude::*;

pub fn fov(state: &mut State, commands: &mut CommandBuffer) {
    let mut views = state.ecs.query::<(&Point, &mut FieldOfView)>();
    let player_location = state.player_location.clone(); //temp variable to be overwritten

    let map = state
        .localmaps
        .get(&player_location)
        .expect("failed to extract player's current location from localmaps hashmap.");
    //will need to provide this system the current map the player is in

    views //this needs to be commented better I'm still not sure how exactly this shit works
        .iter()
        .filter(|(_, (_, fov))| fov.is_dirty)
        .for_each(|(_, (pos, fov))| {
            fov.visible_tiles = field_of_view_set(*pos, fov.radius, map);
            fov.is_dirty = false;
        });
}
