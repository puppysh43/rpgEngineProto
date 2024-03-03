use crate::prelude::*;
//this function will display all of the carried items in the inventory that aren't equipped.
pub fn view_inventory(state: &mut State) {
    let mut carried_item_query = state.ecs.query::<(&Carried, &Name)>();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(MAIN_LAYER);
    draw_batch.cls();
    draw_batch.target(EFFECTS_LAYER);
    //draw the frame
    draw_batch.draw_hollow_double_box(
        Rect::with_size(0, 0, SCREEN_WIDTH - 1, SCREEN_HEIGHT - 1),
        ColorPair::new(ALICEBLUE, BLACK),
    );
    let mut linecount = 1;
    for (_id, (carried_component, name)) in carried_item_query.iter() {
        if carried_component.0 == state.player {
            draw_batch.print_color(
                Point::new(1, linecount),
                name.0.clone(),
                ColorPair::new(LIMEGREEN, BLACK),
            );
            linecount += 1;
        }
    }
    //this will be similar to how inventory is handled in stuff like caves of qud
    draw_batch.submit(5000).expect("Batch Error");
}
