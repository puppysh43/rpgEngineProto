use crate::prelude::*;

pub fn map_transitions(state: &mut State, commands: &mut CommandBuffer) {
    match state.is_in_overworld {
        true => {
            //this will iterate through all of the "wants to enter location" message of intent
            //entities
            for (_, moi) in state.ecs.query_mut::<&WantsToEnterLocation>() {
                let entity = moi.entity;
                let pos_idx = map_idx(moi.pos.x, moi.pos.y);
                let location = LocationID::FirstTown; //filler location b/c I'm too silly with it to learn how to use options
                                                      //this location will NOT be used unless there's a valid location detected in the player's current position.
                let tile = state.worldmap.tiles[pos_idx];
                //need to see if the tile contains a locationID and then push that locationID onto
                //the entity from the message of intent + set their 3D point component to 0,0,0
            }
        }
        false => {
            //instead of doing all this horseshit this function will just iterate through all the map transition
            //messages of intent and process them.
            for (_, moi) in state.ecs.query_mut::<&WantsToChangeMap>() {
                //get the position of the player, the entity changing map, and the cardinal direction
                //they're going in.
                //then do some math to see if they're trying to exit a location, if they're near the
                //in the same tile as an elevator or stairs that would let that entity move up.
            }
        }
    }
}
