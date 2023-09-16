use crate::prelude::*;

#[system] //macro that does all the boilerplate to mark this function as a system
#[read_component(Point)]
#[read_component(Name)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_pos: &Point) {
    let mut positions = <(Entity, &Point, &Name)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let map_pos = *mouse_pos;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(TOOLTIP_LAYER);
    let player_fov = fov.iter(ecs).nth(0).unwrap();
    positions
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == map_pos && player_fov.visible_tiles.contains(&pos))
        .for_each(|(entity, _, name)| {
            let screen_pos = *mouse_pos * 3;
            let display =
                if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                    format!("{} : {} hp", &name.0, health.current)
                } else {
                    name.0.clone()
                };
            draw_batch.print(screen_pos, &display);
        });
    draw_batch.submit(10100).expect("Batch error");
}
