use crate::prelude::*;

pub fn movement(state: &mut State, commands: &mut CommandBuffer) {
    let player = state.ecs.query::<&Player>().iter().nth(0).unwrap().0;
    for (entity, want_move) in state.ecs.query::<&WantsToMove>().iter() {
        if state.map.can_enter_tile(want_move.destination) {
            commands.insert_one(want_move.entity, want_move.destination);

            if let Ok(entry) = state.ecs.entity(want_move.entity) {
                //need to get the entity then get the component ref then map it for the value THEN dereference it
                if let fov = entry
                    .get::<&FieldOfView>()
                    .expect("Entity has no field of view component.")
                //need to get the entity then the component ref then deref it
                {
                    commands.insert_one(want_move.entity, fov.clone_dirty());

                    if entry.entity() == player
                    // (1)
                    {
                        fov.visible_tiles.iter().for_each(|pos| {
                            // (2)
                            state.map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
                        });
                    }
                }
            }
        }
        commands.despawn(entity);
    }
}
