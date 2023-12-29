use crate::prelude::*;
use crate::systems::library::*;

pub fn aiming_ranged(state: &mut State, commands: &mut CommandBuffer) {
    //this is gonna handle the player input for when they're aiming a ranged weapon.
    //player will aim the reticule and then be able to confirm the attack with f again
    //later the player will be able to select what kind of ranged attack they do w/ shift and alt
    let (player_localmap, player_mapscreen, player_pos, current_mapscreen_data) =
        get_player_info_and_map(state);
    let key = state.key.expect("this should never happen");
    let shift = state.shift;
    let control = state.control;
    let alt = state.alt;
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
        VirtualKeyCode::Escape => {
            //this exits the looking turnstate and also deletes the reticule entity.
            for (reticule, _) in state.ecs.query_mut::<With<&Point, &Reticule>>() {
                commands.despawn(reticule);
            }

            state.controlstate = ControlState::Default;
            Point::new(0, 0)
        }
        VirtualKeyCode::F => {
            //this will then split off into no extra key being pressed, w/ shift, and w/ cntrl
            //check if the player's equipped weapon has the ability for that
            //if so then check if the position of the reticule is the same as an entity at that position in the same mapscreen
            //query for all entities in the current mapscreen
            let mut all_entities = state
                .ecs
                .query::<(&CurrentLocalMap, &Point3D, &Point, &Health, &Skills)>();
            all_entities
                .iter()
                .filter(|(_, (localmap, mapscreen, _, _, _))| {
                    localmap.0 == player_localmap && **mapscreen == player_mapscreen
                });
            if !control && !shift && !alt {
                let mut shooter_skills = Skills::new_blank();
                let mut player_weapon = EquippedRangedWeapon(None);
                let mut shooter_query = state
                    .ecs
                    .query::<With<(&EquippedRangedWeapon, &Skills), &Player>>();
                for (_, (equipped_ranged_weapon, skills)) in shooter_query.iter() {
                    player_weapon = *equipped_ranged_weapon;
                    shooter_skills = *skills;
                }
                //this will always do a single shot
                //do a big query of all the necessary information
                //check if the player has a weapon, if the weapon can do the selected attack type
                //and if the reticule is over an entity
                //check if there's any walls in a line between the player and the shooter and then
                //check if there's any entities in the line before the reticule and instead have the ranged moi target them
                //if so spawn in an MOI for the appropriate ranged attack
                if player_weapon.0.is_some() {
                    commands.spawn((
                        //push the moi into the ECS
                        (),
                        WantsToRangedAttack {
                            shooter: state.player.clone(),
                            attack_type: RangedAttackType::SingleShot,
                            shooter_weapon: player_weapon.0.unwrap(),
                            shooter_skills,
                            target,
                            target_skills,
                            target_armor,
                            distance,
                            is_target_ducking,
                            is_in_cover,
                        },
                    ));
                }
            }
            if !control && shift && !alt {
                //this will do a two shot burst
            }
            if control && !shift && !alt {
                //this will do a three shot burst
            }
            if !control && !shift && alt {
                //this will do a full auto attack
            }

            Point::new(0, 0)
        }

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
