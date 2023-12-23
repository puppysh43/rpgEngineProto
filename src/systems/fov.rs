use crate::prelude::*;

pub fn fov(state: &mut State, commands: &mut CommandBuffer) {
    let mut views = state.ecs.query::<(&Point, &mut FieldOfView)>();

    let mut player_localmap = LocalMapID::FirstTown; //temp variable to be overwritten
    let mut player_mapscreen = Point3D::new(0, 0, 0);
    for (_, (current_localmap, mapscreen)) in state
        .ecs
        .query::<With<(&CurrentLocalMap, &Point3D), &Player>>()
        .iter()
    {
        player_localmap = current_localmap.0;
        player_mapscreen = *mapscreen;
    }
    let map = state
        .localmaps
        .get(player_localmap)
        .get_mapscreen(player_mapscreen);
    //will need to provide this system the current map the player is in

    views //this needs to be commented better I'm still not sure how exactly this shit works
        .iter()
        .filter(|(_, (_, fov))| fov.is_dirty)
        .for_each(|(_, (pos, fov))| {
            fov.visible_tiles = field_of_view_set(*pos, fov.radius, &map);
            fov.is_dirty = false;
        });
}
