use crate::prelude::*;

// #[system]
// #[read_component(Point)]
// #[read_component(MovingRandomly)]
// #[read_component(Health)]
// #[read_component(Player)]
// pub fn random_move(ecs: &SubWorld, commands: &mut CommandBuffer)
//TODO will definitely need to take a look at this later but I can't be asked rn
pub fn random_move(state: &mut State) {
    let mut commands = CommandBuffer::new();
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
                if ecs //I think this checks the ecs for if the victim of the attack is the player
                    .entry_ref(victim)
                    .unwrap()
                    .get_component::<Player>() //no clue how to get an entity with its components or anything
                    .is_ok()
                {
                    commands.spawn((
                        (),
                        WantsToAttack {
                            attacker: entity,
                            victim: victim,
                        },
                    ));
                }
                attacked = true;
            });

        if !attacked {
            commands.spawn((
                (),
                WantsToMove {
                    entity: entity,
                    destination,
                },
            ));
        }
    });
    commands.run_on(&mut state.ecs);
}
