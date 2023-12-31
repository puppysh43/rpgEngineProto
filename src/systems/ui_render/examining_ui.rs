use crate::prelude::*;
use crate::systems::ui_render::library::*;
//TODO add commmand buffer
const DESC_WIDTH: i32 = 60;
//screen height 35
//screen width 70
//use effects layer
pub fn examine_entity(state: &mut State, commands: &mut CommandBuffer) {
    let mut description = String::new(); //variable to hold the description to be displayed
    let mut line_num: i32 = 3;
    for (entity, long_desc) in state
        .ecs
        .query::<With<&LongDescription, &Examining>>()
        .iter()
    {
        description = long_desc.0.clone(); //grab the long description of any item tagged as being examined
    }
    let formatted_text = greedy_word_wrap(description, DESC_WIDTH);

    //will need to format the description string so that it can fit on the screen and be pretty

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(MAIN_LAYER);
    draw_batch.cls();
    draw_batch.target(TOOLTIP_LAYER);
    draw_batch.cls();
    draw_batch.target(UI_LAYER);
    draw_batch.cls();
    draw_batch.target(EFFECTS_LAYER);
    draw_batch.cls();

    for line in formatted_text {
        draw_batch.print_color_centered(line_num, line, ColorPair::new(ORANGE, BLACK));
        line_num += 1;
    }

    draw_batch.submit(5000).expect("Batch Error");
}
