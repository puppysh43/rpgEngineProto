use crate::prelude::*;
use crate::systems::library::*;

//TODO will definitely need to take a look at this later but I can't be asked rn
pub fn random_move(state: &mut State, commands: &mut CommandBuffer) {
    let (player_localmap, player_pos3d, _, _) = get_player_info_and_map(state);
    // println!("running the random movement ai system.");
    // let player_index = map_idx(player_pos.x, player_pos.y);
    let player_entity = state.player;
    let mut random_movers = state
        .ecs
        .query::<With<(&CurrentLocalMap, &Point3D, &Point), &MovingRandomly>>(); //maybe switch this to a query with b/c afaik you never actually need the moving randomly component??
    let mut all_entities = state
        .ecs
        .query::<(&CurrentLocalMap, &Point3D, &Point, &Health)>();

    for (random_mover, (_, _, pos)) in random_movers
        .iter()
        .filter(|(_, (loc, pos_3d, _))| loc.0 == player_localmap && **pos_3d == player_pos3d)
    {
        println!("there is a random mover!");
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;

        let mut attacked = false;

        for (target_entity, (_, _, _, _)) in all_entities
            .iter()
            .filter(|(_, (loc, pos_3d, _, _))| loc.0 == player_localmap && **pos_3d == player_pos3d)
            .filter(|(_, (_, _, pos, _))| **pos == destination)
        //for some reason rn this only moves if it's disabled.
        {
            if target_entity == player_entity {
                commands.spawn((
                    (),
                    WantsToAttack {
                        attacker: random_mover,
                        victim: target_entity,
                    },
                ));
                println!("A message of intent to attack was created by a random mover.");
                attacked = true;
            }
        }
        if !attacked {
            commands.spawn((
                (),
                WantsToMove {
                    entity: random_mover,
                    destination,
                },
            ));
            println!("A message of intent to move was created by a random mover.");
        }
    }
}
