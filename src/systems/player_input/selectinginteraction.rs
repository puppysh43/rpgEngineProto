use crate::prelude::*;
use crate::systems::library::*;
use crate::systems::player_input::library::*;

pub fn selecting_interaction(state: &mut State, commands: &mut CommandBuffer) {
    let key = state.key.expect("this should never happen.");
    let (player_localmap, player_mapscreen, player_pos, _) = get_player_info_and_map(state);

    let selection_delta: Point = match key {
        VirtualKeyCode::Left => Point::new(-1, 0),
        VirtualKeyCode::Right => Point::new(1, 0),
        VirtualKeyCode::Up => Point::new(0, -1),
        VirtualKeyCode::Down => Point::new(0, 1),

        //more advanced movement w/ numpad including diagonals
        VirtualKeyCode::Numpad4 => Point::new(-1, 0), //move west
        VirtualKeyCode::Numpad6 => Point::new(1, 0),  //move east
        VirtualKeyCode::Numpad8 => Point::new(0, -1), //move north
        VirtualKeyCode::Numpad2 => Point::new(0, 1),  //move south
        VirtualKeyCode::Numpad7 => Point::new(-1, -1), //move northwest
        VirtualKeyCode::Numpad9 => Point::new(1, -1), //move northeast
        VirtualKeyCode::Numpad3 => Point::new(1, 1),  //move southeast
        VirtualKeyCode::Numpad1 => Point::new(-1, 1), //move southwest
        VirtualKeyCode::Escape => {
            //in the future this will exit out of selecting an interaction without
            Point::new(0, 0)
        }
        _ => Point::new(0, 0),
    };
    //take selection delta and add it to player pos and then check all entities with an interaction component
    //to see if they have the same position as the selection point (from the delta and player_pos)
    //if its the same position then add the active interaction menu component
    let selection_pos = Point::new(
        player_pos.x + selection_delta.x,
        player_pos.y + selection_delta.y,
    );
    for (id, (_, _, pos)) in state
        .ecs
        .query::<With<(&CurrentLocalMap, &Point3D, &Point), &InteractionMenuKey>>()
        .iter()
        .filter(|(_, (localmap, mapscreen, _))| {
            localmap.0 == player_localmap && **mapscreen == player_mapscreen
        })
    {
        if pos == &selection_pos {
            commands.insert_one(id, ActiveInteractionMenu);
            state.controlstate = ControlState::InteractionMenu(InteractionMenuState::MakingChoice);
            state.uistate = UiState::InteractionMenu;
        } else {
            state.controlstate = ControlState::Default;
        }
    }
}
