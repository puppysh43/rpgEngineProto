use crate::prelude::*;

pub fn viewing_log(state: &mut State) {
    //this will just let the player scroll with various arrow keys and exit with the escape key
    let key = state.key.expect("this should never happen.");
    match key {
        VirtualKeyCode::Escape => {
            state.controlstate = ControlState::Default;
            state.uistate = UiState::Default;
            //this will exit the log screen
        }
        VirtualKeyCode::Up | VirtualKeyCode::Numpad8 => {
            //this will scroll up the log.
        }
        VirtualKeyCode::Down | VirtualKeyCode::Numpad2 => {
            //this will scroll down the log
        }
        _ => {
            //filler
        }
    }
}
