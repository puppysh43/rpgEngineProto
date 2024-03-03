use crate::prelude::*;

pub fn map_transitions(state: &mut State, commands: &mut CommandBuffer) {
    //instead of doing all this horseshit this function will just iterate through all the map transition
    //messages of intent and process them.
    for (moi_id, moi) in state.ecs.query_mut::<&WantsToChangeMap>() {
        //get the position of the player, the entity changing map, and the cardinal direction
        //they're going in.
        //then do some math to see if they're trying to exit a location, if they're near the
        //in the same tile as an elevator or stairs that would let that entity move up.
        let entity = moi.entity;
        let entity_pos = moi.pos;
        let direction = moi.cardinal_direction;
        let start_3dpos = moi.map_pos;
        let localmap_id = moi.current_localmap;
        match direction {
            //TODO optimize - move any repeated code between all cases to outside the match statement
            CardinalDirection::North
            | CardinalDirection::East
            | CardinalDirection::South
            | CardinalDirection::West => {
                //entities can only get a want to change map moi from entering transition tiles so only the most basic of sanity checking is necessary.
                //just make sure there's a map file in the location hashmap with the corresponding new 3D point
                let localmap = state.localmaps.get(localmap_id);
                // let new_3dpos = *start_3dpos + delta_from_direction(direction);//I want to be able to do this, find out a way to do this
                let delta_3d = delta_from_direction(direction);
                let new_3dpos = Point3D::new(
                    start_3dpos.x + delta_3d.x,
                    start_3dpos.y + delta_3d.y,
                    start_3dpos.z + delta_3d.z,
                );
                //if there's a valid map in that direction
                if localmap.check_mapscreen(new_3dpos) {
                    commands.insert_one(entity, new_3dpos); //update the player's 3d map position in the location
                                                            //depending on what direction they're doing they need to have one of their point coordinates shifted to the opposite edge
                                                            //remember to put them 1 tile out so they don't get stuck
                    match direction {
                        CardinalDirection::North => {
                            commands.insert_one(entity, Point::new(entity_pos.x, MAP_HEIGHT - 2));
                        }
                        CardinalDirection::East => {
                            commands.insert_one(entity, Point::new(1, entity_pos.y));
                        }
                        CardinalDirection::South => {
                            commands.insert_one(entity, Point::new(entity_pos.x, 1));
                        }
                        CardinalDirection::West => {
                            commands.insert_one(entity, Point::new(MAP_WIDTH - 2, entity_pos.y));
                        }
                        _ => {}
                    }
                }
            }
            CardinalDirection::Up | CardinalDirection::Down => {
                //the player can request going up or down whenever they want so 3 checks need to occur
                //1) is there a map above the player and is their Z coordinate 0. if so then wanting to go up is also a request to leave the
                //current location and switch to being in the overworld.
                //2) ignoring the possibility of leaving the current location you will need to check both if there's an appropriate map in that direction
                //AND if the player is on a tiletype that allows for vertical movement such as stairs or an elevator, you do NOT want players to be able
                //to magically phase through ceilings and floors at the press of a button!!
                let location = state.localmaps.get(localmap_id);
                let current_mapscreen = location.get_mapscreen(start_3dpos);
                let delta_3d = delta_from_direction(direction);
                let new_3dpos = Point3D::new(
                    start_3dpos.x + delta_3d.x,
                    start_3dpos.y + delta_3d.y,
                    start_3dpos.z + delta_3d.z,
                );
                let entity_index = map_idx(entity_pos.x, entity_pos.y);
                if direction == CardinalDirection::Down {
                    //if the direction is down you just need to check that there's a map below the player and that their position matches up with a downstair
                    if location.check_mapscreen(new_3dpos)
                        && current_mapscreen.tiles[entity_index] == TileType::StairDown
                    {
                        commands.insert_one(entity, new_3dpos);
                        //overwrite the old 3d position to move them down w/in the location!
                        //no need to edit their 2d position b/c they're just going up or down
                    }
                } else if direction == CardinalDirection::Up {
                    //check if there's a map above them and they're on an up stair if so do the same as above
                    if location.check_mapscreen(new_3dpos)
                        && current_mapscreen.tiles[entity_index] == TileType::StairUp
                    {
                        commands.insert_one(entity, new_3dpos);
                    } else if start_3dpos.z == 0 {
                        //if the player is on ground level and they aren't on stairs
                        //and they wanna move up, then take them to the overworld
                        commands.remove_one::<&CurrentLocalMap>(entity);
                        commands.remove_one::<&Point3D>(entity);
                        commands.remove_one::<&Point>(entity);
                        state.map_state = MapState::WorldMap;
                        state.controlstate = ControlState::InWorldMap;
                    }
                }
            }
        }
        commands.despawn(moi_id);
    }
}

fn delta_from_direction(direction: CardinalDirection) -> Point3D {
    match direction {
        CardinalDirection::North => Point3D::new(0, 1, 0),
        CardinalDirection::East => Point3D::new(1, 0, 0),
        CardinalDirection::South => Point3D::new(0, -1, 0),
        CardinalDirection::West => Point3D::new(-1, 0, 0),
        CardinalDirection::Up => Point3D::new(0, 0, 1),
        CardinalDirection::Down => Point3D::new(0, 0, -1),
    }
}
