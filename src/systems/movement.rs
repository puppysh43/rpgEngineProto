use crate::prelude::*;

pub fn movement(state: &mut State, commands: &mut CommandBuffer) {
    if !state.is_in_overworld {
        let player = state.player.clone();
        let player_location = state.player_location.clone();
        let mut map = state
            .localmaps
            .get(&player_location)
            .expect("failed to extract player's current location from localmaps hashmap.")
            .clone();

        for (entity, want_move) in state.ecs.query::<&WantsToMove>().iter() {
            if map.can_enter_tile(want_move.destination) {
                commands.insert_one(want_move.entity, want_move.destination);

                if let Ok(entry) = state.ecs.entity(want_move.entity) {
                    //need to get the entity then get the component ref then map it for the value THEN dereference it
                    if let fov = entry
                        .get::<&FieldOfView>()
                        .expect("Entity has no field of view component.")
                    //need to get the entity then the component ref then deref it
                    {
                        commands.insert_one(want_move.entity, fov.clone_dirty());

                        if entry.entity() == player {
                            fov.visible_tiles.iter().for_each(|pos| {
                                map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
                            });
                        }
                    }
                }
            }
            commands.despawn(entity);
        }
        state.localmaps.insert(player_location, map);
    }
}
