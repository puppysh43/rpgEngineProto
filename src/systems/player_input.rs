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
                        spawn_reticule(commands, player_pos);
                        state.controlstate = ControlState::Looking;
                        Point::new(0, 0)
                    }
                    _ => Point::new(0, 0),
                };
            }

            ControlState::Looking => {
                //look at examples from earlier in the book on how to move an object w/out using message of intent
                println!("Looking control state.");
                //player will be able to move the reticule with the numpad, print a brief description to the log with v and view a full screen description with V
                //escape will let the player exit looking mode and go back to default mode
                reticule_delta = match key {
                    VirtualKeyCode::Numpad4 => Point::new(-1, 0), //move west
                    VirtualKeyCode::Numpad6 => Point::new(1, 0),  //move east
                    VirtualKeyCode::Numpad8 => Point::new(0, -1), //move north
                    VirtualKeyCode::Numpad2 => Point::new(0, 1),  //move south
                    VirtualKeyCode::Numpad7 => Point::new(-1, -1), //move northwest
                    VirtualKeyCode::Numpad9 => Point::new(1, -1), //move northeast
                    VirtualKeyCode::Numpad3 => Point::new(1, 1),  //move southeast
                    VirtualKeyCode::Numpad1 => Point::new(-1, 1), //move southwest
                    VirtualKeyCode::V => {
                        println!(
                            "This will pop up the full detailed description as its own window!"
                        );
                        Point::new(0, 0)
                    }
                    VirtualKeyCode::Return | VirtualKeyCode::NumpadEnter => {
                        println!("This will print the entity description to the log!");
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
                    //to figure out to move reticule investigate TurnBasedGames/turnbased/src/systems/player_input.rs
                    //copy over
                }
            }
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
                commands.spawn((
                    //and create a log message just for testing purposes, will remove later
                    (),
                    AddToLog {
                        body: "You have moved!".to_string(),
                    },
                ));
            }
        };

        //This checks the reticule_delta and moves it around the screen!
        if reticule_delta.x != 0 || reticule_delta.y != 0 {
            for (reticule_id, reticule_pos) in
                state.ecs.query::<With<&mut Point, &Reticule>>().iter()
            {
                let new_pos = *reticule_pos + reticule_delta;
                commands.remove_one::<Point>(reticule_id);
                commands.insert_one(reticule_id, new_pos);
            }
        }

        //This match statement ensures the turn only continues if the player is done with inputs e.g targeting ranged attack, looking around, etc
        match control_state {
            ControlState::Default => state.turnstate = TurnState::PcTurn,
            ControlState::Looking => state.turnstate = TurnState::AwaitingInput,
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
