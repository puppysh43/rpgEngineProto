use crate::prelude::*;
use crate::systems::ui_render::library::*;

pub fn view_log(state: &mut State) {
    //make the drawbatch needed to print to the virtual terminal
    let mut draw_batch = DrawBatch::new();
    //clear the layers that may have the gameworld displayed
    draw_batch.target(MAIN_LAYER);
    draw_batch.cls();
    draw_batch.target(EFFECTS_LAYER);
    //draw the frame
    draw_batch.draw_hollow_double_box(
        Rect::with_size(0, 0, SCREEN_WIDTH - 1, SCREEN_HEIGHT - 1),
        ColorPair::new(ALICEBLUE, BLACK),
    );
    //then draw as much of the log as will print on the screen.
    let mut free_space = SCREEN_HEIGHT - 1;
    //this will print most recent bottom to top instead of top to bottom like the traditional log

    //this does not work will need to troubleshoot later
    state.log.iter().rev().for_each(|message| {
        if free_space > 0 {
            for line in greedy_word_wrap(message.clone(), SCREEN_WIDTH - 2) {
                draw_batch.print_color(
                    Point::new(1, free_space),
                    line,
                    ColorPair::new(GREEN, BLACK),
                );
                free_space -= 1;
            }
        }
    });

    draw_batch.submit(5000).expect("Batch Error");
}
