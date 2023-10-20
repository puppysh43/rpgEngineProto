use crate::prelude::*;

// #[system(for_each)]
// #[read_component(Player)]
// #[read_component(FieldOfView)]
//this is a for each sustem meaning
// pub fn movement(
// entity: &Entity,
// want_move: &WantsToMove,
// #[resource] map: &mut Map,
// ecs: &mut SubWorld,
// commands: &mut CommandBuffer,
// )
pub fn movement(state: &mut State) {
    let commands = &mut CommandBuffer::new();
    for (entity, want_move) in state.ecs.query::<&WantsToMove>().iter() {
        if state.map.can_enter_tile(want_move.destination) {
            commands.insert_one(want_move.entity, want_move.destination);

            if let Ok(entry) = ecs.entry_ref(want_move.entity) {
                if let Ok(fov) = entry.get_component::<FieldOfView>() {
                    commands.add_component(want_move.entity, fov.clone_dirty());

                    if entry.get_component::<Player>().is_ok()
                    // (1)
                    {
                        fov.visible_tiles.iter().for_each(|pos| {
                            // (2)
                            map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
                        });
                    }
                }
            }
        }
        commands.remove(*entity);
    }
    commands.run_on(&mut state.ecs);
}
