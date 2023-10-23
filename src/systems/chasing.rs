use crate::prelude::*;

pub fn chasing(state: &mut State, commands: &mut CommandBuffer) {
    let mut movers = state.ecs.query::<(&Point, &ChasingPlayer, &FieldOfView)>();
    let mut positions = state.ecs.query::<(&Point, &Health)>();
    let mut player = state.ecs.query::<(&Point, &Player)>();

    let player_entity = state.ecs.query::<&Player>().iter().nth(0).unwrap().0; //player entity to check if the victim of an attack is the player
    let player_pos = player.iter().nth(0).unwrap().1 .0;
    let player_idx = map_idx(player_pos.x, player_pos.y);

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &search_targets,
        &state.map,
        1024.0,
    );

    movers.iter().for_each(|(entity, (pos, _, fov))| {
        if !fov.visible_tiles.contains(&player_pos) {
            return;
        }
        let idx = map_idx(pos.x, pos.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, &state.map) {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
            let destination = if distance > 1.2 {
                state.map.index_to_point2d(destination)
            } else {
                *player_pos
            };

            let mut attacked = false;
            positions
                .iter()
                .filter(|(entity, (target_pos, _))| **target_pos == destination)
                .for_each(|(victim, (_, _))| {
                    if victim == player_entity {
                        commands.spawn((
                            (),
                            WantsToAttack {
                                attacker: entity,
                                victim,
                            },
                        ));
                    }
                    attacked = true;
                });

            if !attacked {
                commands.spawn((
                    (),
                    WantsToMove {
                        entity,
                        destination,
                    },
                ));
            }
        }
    });
}
