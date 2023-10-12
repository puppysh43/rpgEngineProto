use crate::prelude::*;
const UI_XSTART: i32 = (MAP_WIDTH * 2) + 1;
const UI_BORDER_X: i32 = MAP_WIDTH * 2;
const LOG_YSTART: i32 = 11;
/*
NOTES ON DRAWING UI
* font is 12x24 width and height so the rows line up but you have twice as much room for text
* might want to break individual sections into their own functions
* you have space for 39 characters between the first border and the edge of the screen
*/
#[system]
#[read_component(Player)]
#[read_component(Health)]
//will later read stuff like items attributes and the log
pub fn ui_render(ecs: &SubWorld, #[resource] log: &mut Vec<String>) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(UI_LAYER);
    //draw the border between the gameplay zone and the UI
    for y in 0..MAP_HEIGHT {
        draw_batch.print_color(
            Point::new(UI_BORDER_X, y),
            "║",
            ColorPair::new(GREEN, BLACK),
        );
    }

    draw_batch.print_color(
        Point::new(UI_XSTART, 0),
        "Health 10/10",
        ColorPair::new(RED, GREEN),
    );

    draw_batch.print_color(
        Point::new(UI_XSTART, 1),
        "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
        ColorPair::new(BLUE, BLACK),
    );

    draw_batch.print_color(
        Point::new(UI_XSTART, 10),
        "Log:",
        ColorPair::new(YELLOW, BLACK),
    );
    //printing the log you'll need to go through the log vec from last entry to first entry
    //you'll need to then go through it one by one and check that the length of the log entry is 38 characters or less and split it into two lines.
    //will need to keep track of how many lines have been printed to ensure that you don't exceed the bounds of the screen
    let mut free_log_space = MAP_HEIGHT - LOG_YSTART;

    log.iter().rev().for_each(|message| {
        if (free_log_space > 1 && message.len() < 39) {
            draw_batch.print_color(
                Point::new(UI_XSTART, free_log_space),
                message,
                ColorPair::new(LIGHTGRAY, BLACK),
            );
            free_log_space -= 1;
        }
    });

    draw_batch.submit(5000).expect("Batch Error");
}
//this system is gonna first grab basic information about the character like health, fatigue points, etc, and then it'll process all the new log messages and add them to the full log for the session
