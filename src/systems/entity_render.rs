use crate::prelude::*;

pub fn entity_render(state: &mut State) {
    let mut renderables = state
        .ecs
        .query::<Without<(&Point, &Render, &CurrentLocation, &Point3D), &Effect>>();
    let mut fov = state.ecs.query::<With<&FieldOfView, &Player>>();
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

    if state.uistate == UiState::Default {
        //only render entities if the player is in the default ui mode (in game not in a menu)
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(MAIN_LAYER);

        let player_fov = fov.iter().nth(0).unwrap().1;

        renderables //this is a really slick little iter chain that I'll consider replacing with a for loop but we shall see.
            .iter()
            .filter(|(_, (_, _, location, _))| location.0 == player_location) //filter out any entities that don't share the player's location
            .filter(|(_, (_, _, _, map_coords))| **map_coords == player_coords)
            .filter(|(_, (pos, _, _, _))| player_fov.visible_tiles.contains(&pos))
            .for_each(|(_, (pos, render, _, _))| {
                draw_batch.set(*pos, render.color, render.glyph);
                //print statement that will list the position and possibly the name
            });

        draw_batch.submit(5000).expect("Batch error");
    }
}
