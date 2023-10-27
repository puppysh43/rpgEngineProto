use crate::prelude::*;

pub fn entity_render(state: &mut State) {
    let mut renderables = state.ecs.query::<Without<(&Point, &Render), &Effect>>();
    let mut fov = state.ecs.query::<With<&FieldOfView, &Player>>();
    if state.uistate == UiState::Default {
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(MAIN_LAYER);

        let player_fov = fov.iter().nth(0).unwrap().1;

        renderables //this is a really slick little iter chain that I'll consider replacing with a for loop but we shall see.
            .iter()
            .filter(|(_, (pos, _))| player_fov.visible_tiles.contains(&pos))
            .for_each(|(_, (pos, render))| {
                draw_batch.set(*pos, render.color, render.glyph);
                //print statement that will list the position and possibly the name
            });

        draw_batch.submit(5000).expect("Batch error");
    }
}
