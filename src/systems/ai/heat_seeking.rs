use crate::prelude::*;
use crate::systems::library::*;

pub fn heat_seeking(state: &mut State, commands: &mut CommandBuffer) {
    let (player_location, player_pos3d, player_pos, current_mapscreen) =
        get_player_info_and_map(state, commands);
    let player_index = map_idx(player_pos.x, player_pos.y);
    let player_entity = state.player;

    let mut heat_seekers = state
        .ecs
        .query::<With<(&CurrentLocation, &Point3D, &Point), &HeatSeeking>>();
    let mut all_entities = state
        .ecs
        .query::<(&CurrentLocation, &Point3D, &Point, &Health)>();

    let search_targets = vec![player_index];
    let dijkstra_map = DijkstraMap::new(
        MAP_WIDTH,
        MAP_HEIGHT,
        &search_targets,
        &current_mapscreen,
        1024.0,
    );
    for (heat_seeker, (_, _, pos)) in heat_seekers
        .iter()
        .filter(|(_, (loc, pos_3d, _))| loc.0 == player_location && **pos_3d == player_pos3d)
    {
        let index = map_idx(pos.x, pos.y);
        if let Some(destination) =
            DijkstraMap::find_lowest_exit(&dijkstra_map, index, &current_mapscreen)
        //the find lowest exit gets the most direct path to the target point and returns an option
        {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, player_pos); //use the pythagoran algo to get the entity's distance from the player
            let destination = if distance > 1.2 {
                //if the player is more than 1.2 away aka not adjacent to the entity
                current_mapscreen.index_to_point2d(destination) //make the destination of the entity the result of the djisktra map search
            } else {
                player_pos
            };
            let mut attacked = false; //bool to keep track of if the entity has attacked anything
            for (target_entity, (_, _, _, _)) in all_entities
                .iter()
                .filter(|(_, (loc, pos_3d, _, _))| {
                    loc.0 == player_location && **pos_3d == player_pos3d
                })
                .filter(|(_, (_, _, pos, _))| **pos == destination)
            {
                if target_entity == player_entity {
                    commands.spawn((
                        (),
                        WantsToAttack {
                            attacker: heat_seeker,
                            victim: target_entity,
                        },
                    ));
                    attacked = true;
                }
                if !attacked {
                    commands.spawn((
                        (),
                        WantsToMove {
                            entity: heat_seeker,
                            destination,
                        },
                    ));
                }
            }
        }
    }
}
