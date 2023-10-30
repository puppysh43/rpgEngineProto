use crate::prelude::*;

pub fn map_transitions(state: &mut State, commands: &mut CommandBuffer) {
    let (player_entity, player_location) = state
        .ecs
        .query::<With<&Location, &Player>>()
        .iter()
        .nth(0)
        .expect("There is no player.");
    let current_map = state.localmaps.get(&player_location.0).unwrap(); //use it to grab the current map
    let mut player_pos = Point::new(0, 0);
    for (_, pos) in state.ecs.query::<With<&Point, &Player>>().iter() {
        player_pos = *pos; //get the player's current position
    }
    let player_idx = map_idx(player_pos.x, player_pos.y); //get the index of the player from their extracted point component
    let player_tile = current_map.tiles[player_idx]; //so you can grab the tile the player currently occupies
                                                     //and run it through a match statement!
    match player_tile {
        TileType::MapPortal { destination } => {
            commands.insert_one(player_entity, destination);
        }
        _ => { //do nothing},
        }
    }
}
