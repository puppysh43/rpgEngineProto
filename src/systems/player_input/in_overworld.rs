use crate::prelude::*;

pub fn in_overworld(state: &mut State, commands: &mut CommandBuffer) {
    let key = state.key.expect("this should never happen.");
    let shift = state.shift;
    let control = state.control;
    let alt = state.alt;

    //filler
    let overworld_delta = match key {
        //simple arrow key movement for beginners or laptop users
        VirtualKeyCode::Left => Point::new(-1, 0),
        VirtualKeyCode::Right => Point::new(1, 0),
        VirtualKeyCode::Up => Point::new(0, -1),
        VirtualKeyCode::Down => Point::new(0, 1),
        //more complex numpad movement for hardcore gamers
        VirtualKeyCode::Numpad4 => Point::new(-1, 0), //move west
        VirtualKeyCode::Numpad6 => Point::new(1, 0),  //move east
        VirtualKeyCode::Numpad8 => Point::new(0, -1), //move north
        VirtualKeyCode::Numpad2 => Point::new(0, 1),  //move south
        VirtualKeyCode::Numpad7 => Point::new(-1, -1), //move northwest
        VirtualKeyCode::Numpad9 => Point::new(1, -1), //move northeast
        VirtualKeyCode::Numpad3 => Point::new(1, 1),  //move southeast
        VirtualKeyCode::Numpad1 => Point::new(-1, 1), //move southwest
        //TODO add stuff like examine or the ability to access menus like character screen inventory etc
        //
        VirtualKeyCode::NumpadAdd => {
            let mut player_token_pos = Point::new(0, 0);
            for (_, token_pos) in state
                .ecs
                .query::<With<&Point, &OverworldPlayerToken>>()
                .iter()
            {
                player_token_pos = *token_pos;
            }
            println!(
                "player_token_pos is: x:{}, y:{}",
                player_token_pos.x, player_token_pos.y
            );
            commands.spawn((
                (),
                WantsToEnterLocation {
                    pos: player_token_pos,
                    entity: state.player.clone(),
                },
            ));
            Point::new(0, 0)
        }
        _ => {
            println!("whatever man.");
            Point::new(0, 0)
        }
    };
    //this checks the player token delta and moves it around the screen
    if overworld_delta.x != 0 || overworld_delta.y != 0 {
        for (overworldtoken_id, overworldtoken_pos) in state
            .ecs
            .query::<With<&Point, &OverworldPlayerToken>>()
            .iter()
        {
            let new_pos = *overworldtoken_pos + overworld_delta;
            commands.insert_one(overworldtoken_id, new_pos);
        }
    }
}
