use crate::prelude::*;

pub fn aiming_ranged(state: &mut State, commands: &mut CommandBuffer) {
    //this is gonna handle the player input for when they're aiming a ranged weapon.
    //player will aim the reticule and then be able to confirm the attack with f again
    //later the player will be able to select what kind of ranged attack they do w/ shift and alt

    let key = state.key.expect("this should never happen");
    let _shift = state.shift;
    let _control = state.control;
    let _alt = state.alt;
    let reticule_delta = match key {
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

        _ => Point::new(0, 0),
    };

    //checks the reticule delta and moves it around the screen
    if reticule_delta.x != 0 || reticule_delta.y != 0 {
        for (reticule_id, reticule_pos) in state.ecs.query::<With<&Point, &Reticule>>().iter() {
            let new_pos = *reticule_pos + reticule_delta; //calculate a new position for the reticule
            if new_pos.x < MAP_WIDTH && new_pos.y < MAP_HEIGHT && new_pos.x >= 0 && new_pos.y >= 0 {
                commands.insert_one(reticule_id, new_pos); //you don't need to actually remove the original component - an entity can only have one component of each type so this will overwrite it no problem
            }
        }
    }
}
