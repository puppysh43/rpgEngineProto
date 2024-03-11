use crate::prelude::*;
use crate::systems::library::*;
//menu navigation moi processing system
pub fn process_interactions(state: &mut State, commands: &mut CommandBuffer) {
    let active_intmenu_option = get_active_interactionmenu(state);
    if active_intmenu_option.is_some() {
        let active_interactionmenu = get_active_interactionmenu(state).unwrap();
        //filler variable for the index you need to access
        let mut index: Option<usize> = None;
        //query to actually get the index for you in the normal MOI processing idiom
        for (moi_id, moi) in state.ecs.query::<&InteractionMenuChoiceMOI>().iter() {
            index = Some(moi.index);
            commands.despawn(moi_id);
        }
        //get the result MOI component from the checks and consequences of the interaction menu option chose
        //as well as letting it make whatever changes it wants to the gamestate.
        if index.is_some() {
            let cnc_result = active_interactionmenu
                .get_entry(index.unwrap())
                .run_checks_and_consequences(state, commands);
            commands.spawn((
                (),
                InteractionMenuResult {
                    choice_result: cnc_result,
                    current_option_index: index.unwrap(),
                },
            ));
            state.controlstate = ControlState::InteractionMenu(InteractionMenuState::ViewingResult);
        }
    }
}
