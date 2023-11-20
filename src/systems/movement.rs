use crate::prelude::*;

pub fn movement(state: &mut State, commands: &mut CommandBuffer) {
    if !state.is_in_overworld {
        let player = state.player.clone();
        let mut player_location = LocationID::FirstTown; //filler to be overwritten later
        let mut player_pos3D = Point3D::new(0, 0, 0);
        for (id, (location_id, pos_3d)) in state
            .ecs
            .query_mut::<With<(&CurrentLocation, &Point3D), &Player>>()
        {
            player_location = location_id.0;
            player_pos3D = *pos_3d;
        }
        let mut map = state
            .locations
            .get(&player_location)
            .expect("failed to extract player's current location from localmaps hashmap.")
            .get_map(player_pos3D)
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
        let mut temp_location = state.locations.get(&player_location).unwrap().clone();
        temp_location.update_map(player_pos3D, map);
        state
            .locations
            .insert(player_location, temp_location.clone());
    }
}
