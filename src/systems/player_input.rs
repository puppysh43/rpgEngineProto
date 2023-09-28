use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
#[write_component(Point)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
    #[resource] control_state: &mut ControlState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    //get all entities with a point component from the ECS and filter out anything that doesn't have a player tag
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
    //get all entities with a point component from the ecs and filter out any that don't have the enemy tag
    // let mut player_positions = <&Point>::query().filter(component::<Player>());
    // let player_position = player_positions.iter(ecs).nth(0).unwrap(); //this is how I would get the player position as a standalone thing
    //This is the current control block match statement
    let mut player_delta = Point::new(0, 0);
    let mut reticule_delta = Point::new(0, 0);
    if let Some(key) = *key {
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
                        println!("You pressed the look key!");
                        //this will create a new entity in the world that's the reticule, spawn it on the player's position, and switch the control state to looking
                        let (_, player_pos) = players //gonna be honest don't quite understand what all this means but it works!
                            .iter(ecs)
                            .find_map(|(entity, pos)| Some((*entity, *pos)))
                            .unwrap(); //get the player's position
                        commands.push((
                            //creates a reticule object in the world
                            Effect,
                            Reticule,
                            player_pos,
                            Render {
                                color: ColorPair::new(CYAN, BLACK),
                                glyph: to_cp437('♥'),
                            },
                        ));
                        *control_state = ControlState::Looking;
                        Point::new(0, 0)
                    }
                    _ => Point::new(0, 0),
                };
            }

            ControlState::Looking => {
                //look at examples from earlier in the book on how to move an object w/out using message of intent
                println!("Looking control state.");
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
                        println!("This will print the monster description eventually!");
                        Point::new(0, 0)
                    }
                    VirtualKeyCode::Escape => {
                        //this exits the looking turnstate and also deletes the reticule entity.
                        let reticule = <Entity>::query()
                            .filter(component::<Reticule>())
                            .iter(ecs)
                            .nth(0)
                            .unwrap();
                        commands.remove(*reticule);
                        *control_state = ControlState::Default;
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

        let (player_entity, destination) = players //destructures the results of the iterator into two different variables
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + player_delta)))
            .unwrap();

        let mut did_something = false;
        if player_delta.x != 0 || player_delta.y != 0 {
            //if the player moved at all
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    did_something = true;

                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });

            if !hit_something {
                did_something = true;
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        };
        if !did_something {
            if let Ok(mut health) = ecs
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current = i32::min(health.max, health.current + 1);
            }
        }

        //This checks the reticule_delta and moves it around the screen!
        if reticule_delta.x != 0 || reticule_delta.y != 0 {
            //this currently crashes the game for some reason!!
            let mut reticules = <&mut Point>::query().filter(component::<Reticule>());
            reticules.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + reticule_delta;
                *pos = destination;
            });
        };

        //This match statement ensures the turn only continues if the player is done with inputs e.g targeting ranged attack, looking around, etc
        match control_state {
            ControlState::Default => *turn_state = TurnState::PlayerTurn,
            ControlState::Looking => *turn_state = TurnState::AwaitingInput,
        }
    }
}
