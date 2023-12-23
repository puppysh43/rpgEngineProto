use crate::prelude::*;
use crate::systems::library::*;

pub fn movement(state: &mut State, commands: &mut CommandBuffer) {
    let player = state.player.clone();

    let (player_localmap, player_mapscreen, player_pos, mapscreen_data) =
        get_player_info_and_map(state);
    let mut map = mapscreen_data.clone();

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
    let mut temp_localmap = state.localmaps.get(player_localmap).clone();
    temp_localmap.update_mapscreen(player_mapscreen, map);
    state
        .localmaps
        .update(player_localmap, temp_localmap.clone());
}
