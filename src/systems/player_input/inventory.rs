use crate::prelude::*;
//this is gonna handle all the inventory stuff :D
pub fn inventory(state: &mut State) {
    let key = state
        .key
        .expect("somehow it failed to get a key from the gamestate");
    match key {
        VirtualKeyCode::Escape => {
            state.controlstate = ControlState::Default;
            state.uistate = UiState::Default;
        }
        _ => {
            //filler to make the match statement happy
        }
    }
}
