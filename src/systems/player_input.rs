use crate::prelude::*;

pub fn player_input(state: &mut State, commands: &mut CommandBuffer) {
    let mut player_pos = Point::new(0, 0); //init the var to store the player's position
    for (_, pos) in state.ecs.query_mut::<With<&Point, &Player>>() {
        //query for the player's position and assign it to the player_pos var
        player_pos = *pos;
    }
    let key = state.key;
    let control_state = state.controlstate;

    let mut player_delta = Point::new(0, 0);
    let mut reticule_delta = Point::new(0, 0);
    if let Some(key) = key {
        match control_state {
            ControlState::Default => {
                player_delta = match key {
                    //simple arrow key movement for beginners or laptop users
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
                    VirtualKeyCode::V => {
                        // let is_reticule
                        // for (_, _) in state.ecs.query::<&Reticule>
                        spawn_reticule(commands, player_pos);
                        state.controlstate = ControlState::Looking;
                        Point::new(0, 0)
                    }
                    _ => Point::new(0, 0),
                };
            }

            ControlState::Looking => {
                //look at examples from earlier in the book on how to move an object w/out using message of intent
                //player will be able to move the reticule with the numpad, print a brief description to the log with v and view a full screen description with V
                //escape will let the player exit looking mode and go back to default mode
                reticule_delta = match key {
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
                    VirtualKeyCode::V => {
                        let mut reticule_pos = Point::new(0, 0); //create a temp variable to store the reticule's position for logic reasons
                        for (_, pos) in state.ecs.query::<With<&Point, &Reticule>>().iter() {
                            //use a simple query to grab the reticule's position
                            reticule_pos = *pos;
                        }

                        for (entity, pos) in
                            state.ecs.query::<With<&Point, &LongDescription>>().iter()
                        //go through all entities with a position and a long description
                        {
                            if *pos == reticule_pos {
                                //if they're in the same place as the reticule
                                commands.insert_one(entity, Examining); //then add an "Examining" tagging component to it so the UI system can pick it up later and display it
                                state.uistate = UiState::ExaminingEntity; //set the UI state to examining entity so it'll be displayed properly
                                state.controlstate = ControlState::ExaminingEntity;
                            }
                        }
                        //this will add the "Examining" component to whatever shares the same position as the reticule and sets the ui state to "examining entity"
                        Point::new(0, 0)
                    }
                    VirtualKeyCode::Return | VirtualKeyCode::NumpadEnter => {
                        let mut reticule_pos = Point::new(0, 0); //create a temp variable to store the reticule's position for logic reasons
                        for (_, pos) in state.ecs.query::<With<&Point, &Reticule>>().iter() {
                            //do a quick query to grab the reticule's position
                            reticule_pos = *pos;
                        }
                        for (_, (pos, short_desc)) in
                            state.ecs.query::<(&Point, &ShortDescription)>().iter()
                        //go through all entities with a position and a short description
                        {
                            if *pos == reticule_pos {
                                //if they're in the same place as the player's reticule
                                commands.spawn((
                                    //send a message to the log with the short description of the entity under the reticule!
                                    (),
                                    AddToLog {
                                        body: short_desc.0.clone(),
                                    },
                                ))
                            }
                        }
                        Point::new(0, 0)
                    }
                    VirtualKeyCode::Escape => {
                        //this exits the looking turnstate and also deletes the reticule entity.
                        for (reticule, _) in state.ecs.query_mut::<With<&Point, &Reticule>>() {
                            commands.despawn(reticule);
                        }

                        state.controlstate = ControlState::Default;
                        Point::new(0, 0)
                    }
                    _ => Point::new(0, 0),
                }
            }
            ControlState::ExaminingEntity => match key {
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
                    println!("you will not be able to leave until you press escape.");
                }
            },
            _ => {
                println!("This shouldn't happen!")
            }
        };

        let mut players = state.ecs.query::<With<&Point, &Player>>(); //query of all the player entities and their point component
        let mut enemies = state.ecs.query::<With<&Point, &Enemy>>(); //query of all the enemy entities and their point component
        let (player_entity, _) = players.iter().next().unwrap();
        let destination = player_pos + player_delta;

        let mut did_something = false;
        if player_delta.x != 0 || player_delta.y != 0 {
            //if the player moved at all
            let mut hit_something = false;

            for (enemy_entity, pos) in enemies.iter() {
                //iterate through all enemies in the world.
                if *pos == destination {
                    //if their position is the same as where the player is moving
                    hit_something = true;
                    did_something = true; //then track that they hit something
                    commands.spawn((
                        //and create an attack message of intent w/ the player as the attacker and the enemy as the victim!
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: enemy_entity,
                        },
                    ));
                }
            }

            if !hit_something {
                //if the player didn't hit anything on the way to their destination
                did_something = true; //note that they did something!
                commands.spawn((
                    //spawn a message of intent entity for moving the player
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        };

        //This checks the reticule_delta and moves it around the screen!
        if reticule_delta.x != 0 || reticule_delta.y != 0 {
            for (reticule_id, reticule_pos) in state.ecs.query::<With<&Point, &Reticule>>().iter() {
                let new_pos = *reticule_pos + reticule_delta; //calculate a new position for the reticule
                commands.insert_one(reticule_id, new_pos); //you don't need to actually remove the original component - an entity can only have one component of each type so this will overwrite it no problem
            }
        }

        //This match statement ensures the turn only continues if the player is done with inputs e.g targeting ranged attack, looking around, etc
        match control_state {
            ControlState::Default => state.turnstate = TurnState::PcTurn,
            ControlState::Looking | ControlState::ExaminingEntity => {
                state.turnstate = TurnState::AwaitingInput
            }
        }
    }
}

fn spawn_reticule(cmd: &mut CommandBuffer, player_pos: Point) {
    cmd.spawn((
        //creates a reticule object in the world
        Effect,
        Reticule,
        player_pos,
        Render {
            color: ColorPair::new(CYAN, BLACK),
            glyph: to_cp437('♥'),
        },
    ));
}
