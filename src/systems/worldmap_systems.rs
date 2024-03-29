use crate::prelude::*;
//this is where the system that does stuff like handling entering locations and calculating random encounters or whatever other worldmap logic will be in here
//will start as one file but may grow into a directory module
pub fn worldmap_systems(state: &mut State, commands: &mut CommandBuffer) {
    //this function handles entering locations in the worldmap
    for (moi_id, moi_data) in state.ecs.query_mut::<&WantsToEnterLocation>() {
        let entity = moi_data.entity;
        let worldpos_idx = map_idx(moi_data.pos.x, moi_data.pos.y);

        // let mut location = LocationID::FirstTown;//filler location until I figure out how to create a null-ish equivalent

        let current_tile = state.worldmap.tiles[worldpos_idx];

        //TODO figure out a better solution so I don't need to keep updating this every time I add a new tiletype that can have a location
        match current_tile {
            WorldTileType::Town(localmap_id) => {
                //then will set the current location of the entity to the location contained in the tile
                //the 3d point component to 0,0,0
                //and the 2d point component will be set to the spawn point
                let spawn_pos = state.localmaps.get(localmap_id).get_spawnpos();
                commands.insert_one(entity, CurrentLocalMap(localmap_id));
                commands.insert_one(entity, Point3D::new(0, 0, 0));
                commands.insert_one(entity, spawn_pos);
                state.map_state = MapState::LocalMap;
                state.controlstate = ControlState::Default;
                //some point in here I need to fix the player's fov and make it dirty so it doesn't get applied to
                //their new map!
            }
            WorldTileType::Dungeon(localmap_id) => {
                let spawn_pos = state.localmaps.get(localmap_id).get_spawnpos();
                commands.insert_one(entity, CurrentLocalMap(localmap_id));
                commands.insert_one(entity, Point3D::new(0, 0, 0));
                commands.insert_one(entity, spawn_pos);
                state.map_state = MapState::LocalMap;
                state.controlstate = ControlState::Default;
            }
            _ => {
                //do nothing b/c there's no locationID in the filetypes
            }
        }

        commands.despawn(moi_id);
    }
}
