use crate::prelude::*;

pub fn map_transitions(state: &mut State, commands: &mut CommandBuffer) {
    let player_location = state.player_location.clone(); //temp variable to be overwritten
    let player_entity = state.player.clone();

    let current_map = state.localmaps.get(&player_location).unwrap(); //use it to grab the current map
    let mut player_pos = Point::new(0, 0);
    for (_, pos) in state.ecs.query::<With<&Point, &Player>>().iter() {
        player_pos = *pos; //get the player's current position
    }
    let player_idx = map_idx(player_pos.x, player_pos.y); //get the index of the player from their extracted point component
    let player_tile = current_map.tiles[player_idx]; //so you can grab the tile the player currently occupies
                                                     //and run it through a match statement!

    //I'll need to check for an "intent to use exit" for portals, stairs, etc
    //but maybe have something resembling

    match player_tile {
        TileType::MapPortal { destination } => {
            commands.insert_one(player_entity, Location(destination.0));
            commands.insert_one(player_entity, destination.1);
            state.player_location = destination.0;
            println!("the player is on top of a map portal!");
        }
        _ => { //do nothing},
        }
    }
}
