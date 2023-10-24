mod default_ui;
mod carried_ui;
mod equipped_ui;

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

pub fn ui_render(state: &mut State) {
    let ui_state = state.uistate;
    match ui_state {
        UiState::Default => default_ui::default_ui(state),
        UiState::ViewingCarried => carried_ui::view_carried(state),
        UiState::ViewingEquipped => equipped_ui::view_equipped(state),
        UiState::ExaminingEntity => 
        UiState::InDialogue =>
        UiState::ViewingJournal =>
        UiState::ViewingLog =>
    }
}
//this system is gonna first grab basic information about the character like health, fatigue points, etc, and then it'll process all the new log messages and add them to the full log for the session
