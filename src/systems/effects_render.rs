use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn effects_render(ecs: &SubWorld) {
    let mut renderables = <(&Point, &Render)>::query().filter(component::<Effect>());
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(EFFECTS_LAYER);

    let player_fov = fov.iter(ecs).nth(0).unwrap();
    renderables
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(pos, render)| {
            draw_batch.set(*pos, render.color, render.glyph);
        });

    draw_batch.submit(5000).expect("Batch error");
}
