use crate::prelude::*;

pub fn end_turn(state: &mut State) {
    let mut player_hp = state.ecs.query::<With<&Health, &Player>>();
    let current_state = state.turnstate.clone();
    let mut new_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PcTurn => TurnState::NpcTurn,
        TurnState::NpcTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    //check to see if the player's dead and if so switch the gamemode to game over.
    player_hp.iter().for_each(|(_, hp)| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
    });

    if current_state == TurnState::NpcTurn {
        state.numberturns += 1;
        let numberturns = state.numberturns;
        // println!("Turn Number: {}", numberturns);
        //temporarily prints out the number of turns. in the future this will be only used internally for stuff like the passage of time
    }

    state.turnstate = new_state;
}
