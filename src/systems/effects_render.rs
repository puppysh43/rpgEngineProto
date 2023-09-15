
use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn effects_render(ecs: &SubWorld) {
    //WHAT I NEED TO DO
    //GET ALL RENDERABLES THAT HAVE THE EFFECTS TAG
    //ONLY RENDER THOSE
    let mut renderables = <(&Point, &Render)>::query();
    let mut draw_batch = DrawBatch::new();
    // draw_batch.target(1);
    draw_batch.target(0);

rednerables
        .iter(ecs)
        .filter()

    /*
    renderables
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(pos, render)| {
            draw_batch.set(*pos, render.color, render.glyph);
        });
*/
    draw_batch.submit(5000).expect("Batch error");
}
