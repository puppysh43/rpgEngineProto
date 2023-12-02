use crate::prelude::*;

pub fn fov(state: &mut State, commands: &mut CommandBuffer) {
    let mut views = state.ecs.query::<(&Point, &mut FieldOfView)>();

    let mut player_location = LocationID::FirstTown; //temp variable to be overwritten
    let mut player_coords = Point3D::new(0, 0, 0);
    for (_, (current_location, coords)) in state
        .ecs
        .query::<With<(&CurrentLocation, &Point3D), &Player>>()
        .iter()
    {
        player_location = current_location.0;
        player_coords = *coords;
    }
    let map = state.locations.get(player_location).get_map(player_coords);
    //will need to provide this system the current map the player is in

    views //this needs to be commented better I'm still not sure how exactly this shit works
        .iter()
        .filter(|(_, (_, fov))| fov.is_dirty)
        .for_each(|(_, (pos, fov))| {
            fov.visible_tiles = field_of_view_set(*pos, fov.radius, &map);
            fov.is_dirty = false;
        });
}
