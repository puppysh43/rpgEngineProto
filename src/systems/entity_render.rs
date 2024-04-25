use crate::prelude::*;
use crate::systems::library::*;

pub fn entity_render(state: &mut State) {
    //get all the info needed to make sure you only render stuff in the same mapscreen worldspace as the player
    let (player_localmap, player_mapscreen, _player_pos, _mapscreen_data) =
        get_player_info_and_map(state);
    //get all possible renderable entities, the data needed to render them, and the data needed to properly filter them
    let mut renderables = state
        .ecs
        .query::<Without<(&Point, &Render, &CurrentLocalMap, &Point3D), &Effect>>();
    //get the player's FOV
    let mut fov = state.ecs.query::<With<&FieldOfView, &Player>>();

    //conditional to make sure we don't render entities while the player is in a menu
    if state.uistate == UiState::Default {
        //create a new drawbatch to submit to the virtual terminal at the end of the tick
        let mut draw_batch = DrawBatch::new();
        //target the main layer (or whichever layer you're printing entities to)
        draw_batch.target(MAIN_LAYER);
        //get the player's fov from the queried data
        let player_fov = fov.iter().nth(0).unwrap().1;

        //iterate through all the renderable components
        for (_id, (pos, render, localmap, pos_3d)) in renderables.iter() {
            //if they're in the same mapscreen as the player and in the player's fov
            if localmap.0 == player_localmap
                && pos_3d == &player_mapscreen
                && player_fov.visible_tiles.contains(pos)
            {
                //send their position and render data to the drawbuffer
                draw_batch.set(*pos, render.color, render.glyph);
            }
        }

        //submit the draw_batch to the virtual terminal
        draw_batch.submit(5000).expect("Batch error");
    }
}
