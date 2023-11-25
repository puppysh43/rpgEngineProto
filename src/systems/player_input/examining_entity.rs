use crate::prelude::*;

pub fn examining_entity(state: &mut State, commands: &mut CommandBuffer) {
    let key = state.key.expect("this should never happen.");
    let shift = state.shift;
    let control = state.control;
    let alt = state.alt;
    match key {
        VirtualKeyCode::Escape => {
            state.controlstate = ControlState::Default;
            state.uistate = UiState::Default;
            for (entity, _) in state.ecs.query::<&Examining>().iter() {
                commands.remove_one::<Examining>(entity); //then remove the examining/being examined tag
            }
            for (entity, _) in state.ecs.query::<&Reticule>().iter() {
                commands.despawn(entity);
            }
        }
        _ => {
            //you will not be able to leave until you press escape
        }
    };
}
