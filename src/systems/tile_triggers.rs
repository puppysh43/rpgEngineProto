use crate::prelude::*;
//this system will be a broad system that contains all the various things that are triggered by a character of the PC being in
//a certain position on a map.
pub fn tile_triggers(state: &mut State, commands: &mut CommandBuffer) {
    //this function will activate all the stuff that needs to activate when
    //an entity steps over the tile
    check_map_transitions(state, commands);
}

fn check_map_transitions(state: &mut State, commands: &mut CommandBuffer) {
    //this function will check if any entities need to be assigned the wants to change map
    //moi. theoretically I should filter it to only stuff in the player's location but who cares
    //I can just go through literally anything with a position at first lmao
    for (entity_id, (pos, map_pos, current_loc)) in state
        .ecs
        .query_mut::<(&Point, &Point3D, &CurrentLocation)>()
    {
        //use queried information to reference the appropriate map to check for map.
        let location = state
            .locations
            .get(&current_loc.0)
            .expect("failed to get location.");
        let map = location.get_map(*map_pos);
        // for tile in map.tiles {
        let tile = map.tiles[map_idx(pos.x, pos.y)];
        match tile {
            TileType::MapTransitionNorth => commands.spawn((
                (),
                WantsToChangeMap {
                    pos: *pos,
                    entity: entity_id,
                    cardinal_direction: CardinalDirection::North,
                    map_pos: *map_pos,
                    current_location: current_loc.0,
                },
            )),
            TileType::MapTransitionEast => commands.spawn((
                (),
                WantsToChangeMap {
                    pos: *pos,
                    entity: entity_id,
                    cardinal_direction: CardinalDirection::East,
                    map_pos: *map_pos,
                    current_location: current_loc.0,
                },
            )),
            TileType::MapTransitionSouth => commands.spawn((
                (),
                WantsToChangeMap {
                    pos: *pos,
                    entity: entity_id,
                    cardinal_direction: CardinalDirection::South,
                    map_pos: *map_pos,
                    current_location: current_loc.0,
                },
            )),
            TileType::MapTransitionWest => commands.spawn((
                (),
                WantsToChangeMap {
                    pos: *pos,
                    entity: entity_id,
                    cardinal_direction: CardinalDirection::West,
                    map_pos: *map_pos,
                    current_location: current_loc.0,
                },
            )),
            _ => {} //do nothing,
        }
        // }
    }
}
