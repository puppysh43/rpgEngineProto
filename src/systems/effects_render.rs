use crate::prelude::*;

pub fn effects_render(state: &mut State) {
    let mut renderables = state.ecs.query::<With<(&Point, &Render), &Effect>>();
    let mut fov = state.ecs.query::<With<&FieldOfView, &Player>>();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(EFFECTS_LAYER);

    let player_fov = fov.iter().nth(0).unwrap().1;
    renderables
        .iter()
        .filter(|(_, (pos, _))| player_fov.visible_tiles.contains(&pos))
        .for_each(|(_, (pos, render))| {
            draw_batch.set(*pos, render.color, render.glyph);
        });

    draw_batch.submit(5000).expect("Batch error");
}
