use crate::prelude::*;

pub fn heat_seeking(state: &mut State, commands: &mut CommandBuffer) {
    let mut player = state
        .ecs
        .query_mut::<With<(&CurrentLocation, &Point3D, &Point), &Player>>();//query containing all the needed player info
    //collection of individual variables to hold different parts of the relevant player info
    let mut player_entity = state.player;
    let mut player_pos = Point::new(0, 0);
    let mut player_pos3d = Point3D::new(0, 0, 0);
    let mut player_location = LocationID::FirstTown; //temp filler location that will be overwritten
    //iterate through the player query to get the relevant information. this generally safer and neater than using nth and unwrap
    for (_, (location, pos_3d, pos)) in player {
        player_location = location.0;
        player_pos3d = *pos_3d;
        player_pos = *pos;
    }
    
    let current_mapscreen = state.locations.get(player_location).get_map(player_pos3d);//use player info to get the current mapscreen.
    let player_index = map_idx(player_pos.x, player_pos.y);

    let mut heat_seekers = state
        .ecs
        .query::<With<(&CurrentLocation, &Point3D, &Point), &HeatSeeking>>()
        .iter()
        .filter(|(_, (loc, pos_3d, _))| loc.0 == player_location && pos_3d == pos_3d);
    let mut all_entities = state
        .ecs
        .query::<(&CurrentLocation, &Point3D, &Point, &Health)>()
        .iter()
        .filter(|(_, (loc, pos_3d, pos, health))| {
            loc.0 == player_location && **pos_3d == player_pos3d
        });
    let search_targets = vec![player_index];
    let dijkstra_map = DijkstraMap::new(
        MAP_WIDTH,
        MAP_HEIGHT,
        &search_targets,
        &current_mapscreen,
        1024.0,
    );
    for (_, (location, pos_3d, pos)) in heat_seekers {
        let index = map_idx(pos.x, pos.y);
        if let Some(destination) =
            DijkstraMap::find_lowest_exit(&dijkstra_map, index, &current_mapscreen)
            //the find lowest exit gets the most direct path to the target point and returns an option
        {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, player_pos);//use the pythagoran algo to get the entity's distance from the player
            let destination = if distance > 1.2 {//if the player is more than 1.2 away aka not adjacent to the entity
                current_mapscreen.index_to_point2d(destination)//make the destination of the entity the result of the djisktra map search
            } else {
                player_pos
            };
            let mut attacked = false;//bool to keep track of if the entity has attacked anything
        }
    }
}

/*

    movers.iter(ecs).for_each(| (entity, pos, _) | {// (5)
        let idx = map_idx(pos.x, pos.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map,
            idx, map)
        {// (6)
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);// (7)
            let destination = if distance > 1.2 {// (8)
                map.index_to_point2d(destination)
            } else {
                *player_pos
            };

            let mut attacked = false;
            positions
                .iter(ecs)
                .filter(|(_, target_pos, _)| **target_pos == destination)
                .for_each(|(victim, _, _)| {
                    if ecs.entry_ref(*victim).unwrap().get_component::<Player>().is_ok() {
                        commands
                            .push(((), WantsToAttack{
                                attacker: *entity,
                                victim: *victim
                            }));
                    }
                    attacked = true;
                });

            if !attacked {
                commands
                    .push(((), WantsToMove{ entity: *entity, destination }));
            }
        }
    });
}

*/
