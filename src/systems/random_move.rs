use crate::prelude::*;

//TODO will definitely need to take a look at this later but I can't be asked rn
pub fn random_move(state: &mut State, commands: &mut CommandBuffer) {
    let player = state.ecs.query::<&Player>().iter().nth(0).unwrap().0; //player entity to check if the victim of an attack is the player
    let mut movers = state.ecs.query::<(&Point, &MovingRandomly)>(); //maybe switch this to a query with b/c afaik you never actually need the moving randomly component??
    let mut positions = state.ecs.query::<(&Point, &Health)>();
    movers.iter().for_each(|(entity, (pos, _))| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;

        let mut attacked = false;
        positions
            .iter()
            .filter(|(_, (target_pos, _))| **target_pos == destination)
            .for_each(|(victim, (_, _))| {
                if victim == player {
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
    });
}
