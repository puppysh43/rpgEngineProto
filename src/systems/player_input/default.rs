use crate::prelude::*;
use crate::systems::library::*;
use crate::systems::player_input::library::*;

pub fn default(state: &mut State, commands: &mut CommandBuffer) {
    let mut player_delta = Point::new(0, 0);
    let key = state.key.expect("this should never happen.");
    let shift = state.shift;
    let _control = state.control;
    let _alt = state.alt;
    let player_entity = state.player.clone();
    let (player_localmap, player_mapscreen, player_pos, _) = get_player_info_and_map(state);

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
        VirtualKeyCode::NumpadSubtract => {
            //will send some kind of message of intent, either having a separate one for each direction or with one including
            commands.spawn((
                (),
                WantsToChangeMap {
                    pos: player_pos,
                    entity: player_entity,
                    cardinal_direction: CardinalDirection::Up,
                    map_pos: player_mapscreen,
                    current_localmap: player_localmap,
                },
            ));
            Point::new(0, 0)
        }
        VirtualKeyCode::NumpadAdd => {
            commands.spawn((
                (),
                WantsToChangeMap {
                    pos: player_pos,
                    entity: player_entity,
                    cardinal_direction: CardinalDirection::Down,
                    map_pos: player_mapscreen,
                    current_localmap: player_localmap,
                },
            ));
            Point::new(0, 0)
        }
        VirtualKeyCode::V => {
            spawn_reticule(commands, player_pos);
            state.controlstate = ControlState::Looking;
            Point::new(0, 0)
        }
        VirtualKeyCode::F => {
            spawn_reticule(commands, player_pos);
            state.controlstate = ControlState::AimingRanged;
            Point::new(0, 0)
        }
        VirtualKeyCode::M => {
            if shift {
                state.controlstate = ControlState::ViewingLog;
                state.uistate = UiState::ViewingLog;
            }
            Point::new(0, 0)
        }
        VirtualKeyCode::G => {
            //get all items and then filter out any not in the same mapscreen as the player
            let mut all_items = state
                .ecs
                .query::<With<(&CurrentLocalMap, &Point3D, &Point), &Item>>();
            let valid_items = all_items.iter().filter(|(_, (localmap, mapscreen, _))| {
                localmap.0 == player_localmap && **mapscreen == player_mapscreen
            });
            //go through all valid items and see if there's one on the same tile as the player
            for (item_id, (_, _, item_pos)) in valid_items {
                if item_pos.x == player_pos.x && item_pos.y == player_pos.y {
                    //to add items to the player's inventory we just need to
                    commands.insert_one(item_id, Carried(player_entity));
                    commands.remove_one::<Point>(item_id);
                }
            }
            Point::new(0, 0)
        }
        VirtualKeyCode::I => {
            //this will open up the inventory
            state.controlstate = ControlState::Inventory;
            state.uistate = UiState::ViewingInventory;
            Point::new(0, 0)
        }
        VirtualKeyCode::Space => {
            //this will enter interaction selection
            state.controlstate = ControlState::SelectingInteraction;
            Point::new(0, 0)
        }
        _ => Point::new(0, 0),
    };
    //end of key match statement

    let mut players = state.ecs.query::<With<&Point, &Player>>(); //query of all the player entities and their point component
    let mut enemies = state.ecs.query::<With<&Point, &Enemy>>(); //query of all the enemy entities and their point component
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
}
