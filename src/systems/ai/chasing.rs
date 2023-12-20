use crate::prelude::*;
use crate::systems::library::*;

pub fn chasing(state: &mut State, commands: &mut CommandBuffer) {
    let mut movers = state.ecs.query::<(&Point, &ChasingPlayer, &FieldOfView)>();
    let mut positions = state.ecs.query::<(&Point, &Health, &Location)>();
    let mut player = state.ecs.query::<(&Point, &Player)>();
    let (player_location, player_pos3d, player_pos, current_mapscreen) =
        get_player_info_and_map(state, commands);
    let player_index = map_idx(player_pos.x, player_pos.y);
    let player_entity = state.player;

    let mut chasers = state
        .ecs
        .query::<With<(&CurrentLocation, &Point3D, &Point, &FieldOfView), &ChasingPlayer>>();
    let mut all_entities = state
        .ecs
        .query::<(&CurrentLocation, &Point3D, &Point, &Health)>();

    let search_targets = vec![player_index];
    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &search_targets,
        &current_mapscreen,
        1024.0,
    );

    for (chaser, (_, _, pos, fov)) in chasers
        .iter()
        .filter(|(_, (loc, pos_3d, _, _))| loc.0 == player_location && **pos_3d == player_pos3d)
    {
        //optimize this function by breaking if the player's position isn't in the npc's fov
        if !fov.visible_tiles.contains(&player_pos) {
            return;
        }
        let index = map_idx(pos.x, pos.y);
        if let Some(destination) =
            DijkstraMap::find_lowest_exit(&dijkstra_map, index, &current_mapscreen)
        {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, player_pos);
            let destination = if distance > 1.2 {
                current_mapscreen.index_to_point2d(destination)
            } else {
                player_pos
            };
            let mut attacked = false;

            for (target_entity, (_, _, _, _)) in all_entities
                .iter()
                .filter(|(_, (loc, pos_3d, _, _))| {
                    loc.0 == player_location && **pos_3d == player_pos3d
                })
                .filter(|(_, (_, _, pos, _))| **pos == destination)
            //if the position of the entity is where the npc is moving check for if they should attack or not
            {
                //if the target of the destination is the player do an attack
                if target_entity == player_entity {
                    //make an attack MOI if the targeted entity is the player
                    commands.spawn((
                        (),
                        WantsToAttack {
                            attacker: chaser,
                            victim: target_entity,
                        },
                    ));
                    attacked = true;
                }
            }
            if !attacked {
                commands.spawn((
                    (),
                    WantsToMove {
                        entity: chaser,
                        destination,
                    },
                ));
            }
        }
        //if the chasing npc doesn't attack then it just moves
    }
}
