//this is gonna be the default ui render for stuff like the log help et

use crate::prelude::*;
use crate::systems::ui_render::library::*;

pub fn default_ui(state: &mut State) {
    //HAVE ALL NECESSARY QUERIES HERE
    let player_health = state.ecs.query_one_mut::<&Health>(state.player).unwrap();
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
    //display the player's current health
    let health_element = format!("Health {}/{}", player_health.current, player_health.max);
    draw_batch.print_color(
        Point::new(UI_XSTART, 0),
        &health_element,
        ColorPair::new(RED, GREEN),
    );

    // match state.controlstate {
    // ControlState::Default => todo!(),
    // ControlState::Looking => todo!(),
    // ControlState::ExaminingEntity => todo!(),
    // }

    //don't know what this was for
    // draw_batch.print_color(
    // Point::new(UI_XSTART, 1),
    // "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    // ColorPair::new(BLUE, BLACK),
    // );

    draw_batch.print_color(
        Point::new(UI_XSTART, 10),
        "Log:",
        ColorPair::new(YELLOW, BLACK),
    );
    //printing the log you'll need to go through the log vec from last entry to first entry
    //you'll need to then go through it one by one and check that the length of the log entry is 38 characters or less and split it into two lines.
    //will need to keep track of how many lines have been printed to ensure that you don't exceed the bounds of the screen
    let mut free_log_space = LOG_YSTART;

    state.log.iter().rev().for_each(|message| {
        if free_log_space < MAP_HEIGHT {
            for line in greedy_word_wrap(message.clone(), 38) {
                draw_batch.print_color(
                    Point::new(UI_XSTART, free_log_space),
                    line,
                    ColorPair::new(LIGHTGRAY, BLACK),
                );
                free_log_space += 1;
            }
        }
    });

    draw_batch.submit(5000).expect("Batch Error");
}
