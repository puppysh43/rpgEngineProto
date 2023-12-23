use crate::prelude::*;

pub fn entity_render(state: &mut State) {
    let mut renderables = state
        .ecs
        .query::<Without<(&Point, &Render, &CurrentLocalMap, &Point3D), &Effect>>();
    let mut fov = state.ecs.query::<With<&FieldOfView, &Player>>();
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

    if state.uistate == UiState::Default {
        //only render entities if the player is in the default ui mode (in game not in a menu)
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(MAIN_LAYER);

        let player_fov = fov.iter().nth(0).unwrap().1;

        renderables //this is a really slick little iter chain that I'll consider replacing with a for loop but we shall see.
            .iter()
            .filter(|(_, (_, _, localmap, _))| localmap.0 == player_localmap) //filter out any entities that don't share the player's localmap
            .filter(|(_, (_, _, _, map_mapscreen))| **map_mapscreen == player_mapscreen)
            .filter(|(_, (pos, _, _, _))| player_fov.visible_tiles.contains(&pos))
            .for_each(|(_, (pos, render, _, _))| {
                draw_batch.set(*pos, render.color, render.glyph);
                //print statement that will list the position and possibly the name
            });

        draw_batch.submit(5000).expect("Batch error");
    }
}
