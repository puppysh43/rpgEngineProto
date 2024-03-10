use crate::prelude::*;
mod aiming_ranged;
mod default;
mod examining_entity;
mod in_overworld;
mod interaction_menu;
mod inventory;
mod library;
mod looking;
mod viewing_log;

pub fn player_input(state: &mut State, commands: &mut CommandBuffer) {
    let key = state.key;
    let mut control_state = state.controlstate;

    if key.is_some() {
        match control_state {
            ControlState::Default => {
                default::default(state, commands);
            }
            ControlState::InteractionMenu(_) => {
                //
            }
            ControlState::Looking => {
                looking::looking(state, commands);
            }
            ControlState::ExaminingEntity => {
                examining_entity::examining_entity(state, commands);
            }
            ControlState::InWorldMap => {
                in_overworld::in_overworld(state, commands);
            }
            ControlState::AimingRanged => {
                aiming_ranged::aiming_ranged(state, commands);
            }
            ControlState::ViewingLog => {
                viewing_log::viewing_log(state);
            }
            ControlState::Inventory => {
                inventory::inventory(state);
            }
            _ => {
                println!("This shouldn't happen!")
            }
        };

        //This match statement ensures the turn only continues if the player is done with inputs e.g targeting ranged attack, looking around, etc
        control_state = state.controlstate; //controlstate can be changed by the player input functions so we need to update our variable again
        match control_state {
            ControlState::Default => state.turnstate = TurnState::PcTurn,
            ControlState::Looking
            | ControlState::ExaminingEntity
            | ControlState::InWorldMap
            | ControlState::AimingRanged
            | ControlState::ViewingLog
            | ControlState::Inventory => state.turnstate = TurnState::AwaitingInput,
        }
    }
}
