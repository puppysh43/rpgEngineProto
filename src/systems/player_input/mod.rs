use crate::prelude::*;
mod default;
mod examining_entity;
mod in_overworld;
mod library;
mod looking;

pub fn player_input(state: &mut State, commands: &mut CommandBuffer) {
    let key = state.key;
    let control_state = state.controlstate;

    if key.is_some() {
        match control_state {
            ControlState::Default => {
                default::default(state, commands);
            }

            ControlState::Looking => {
                looking::looking(state, commands);
            }
            ControlState::ExaminingEntity => {
                examining_entity::examining_entity(state, commands);
            }
            ControlState::InOverworld => {
                in_overworld::in_overworld(state, commands);
            }
            _ => {
                println!("This shouldn't happen!")
            }
        };

        //This match statement ensures the turn only continues if the player is done with inputs e.g targeting ranged attack, looking around, etc
        match control_state {
            ControlState::Default => state.turnstate = TurnState::PcTurn,
            ControlState::Looking | ControlState::ExaminingEntity | ControlState::InOverworld => {
                state.turnstate = TurnState::AwaitingInput
            }
        }
    }
}
