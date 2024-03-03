mod default_ui;
mod dialogue_ui;
mod equipped_ui;
mod examining_ui;
mod inventory_ui;
mod journal_ui;
mod library;
mod log_ui;

use crate::prelude::*;
/*
NOTES ON DRAWING UI
* font is 12x24 width and height so the rows line up but you have twice as much room for text
* might want to break individual sections into their own functions
* you have space for 39 characters between the first border and the edge of the screen
*/
//consider having it check the gamestate for various flags to see what ui state needs to be done so that
pub fn ui_render(state: &mut State, commands: &mut CommandBuffer) {
    let ui_state = state.uistate;
    match ui_state {
        UiState::Default => default_ui::default_ui(state),
        UiState::ViewingInventory => inventory_ui::view_inventory(state),
        UiState::ViewingEquipped => equipped_ui::view_equipped(state),
        UiState::ExaminingEntity => examining_ui::examine_entity(state, commands),
        UiState::InDialogue => dialogue_ui::render_dialogue(state),
        UiState::ViewingJournal => journal_ui::view_journal(state),
        UiState::ViewingLog => log_ui::view_log(state),
    }
}
