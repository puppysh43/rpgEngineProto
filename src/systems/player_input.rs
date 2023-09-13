use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key : &Option<VirtualKeyCode>,
    #[resource] turn_state : &mut TurnState
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    if let Some(key) = *key {
        let delta = match key {
            //more advanced movement w/ numpad including diagonals
            VirtualKeyCode::Numpad4 => Point::new(-1, 0), //move west
            VirtualKeyCode::Numpad6 => Point::new(1, 0),  //move east
            VirtualKeyCode::Numpad8 => Point::new(0, -1), //move north
            VirtualKeyCode::Numpad2 => Point::new(0, 1),  //move south
            VirtualKeyCode::Numpad7 => Point::new(-1, -1), //move northwest
            VirtualKeyCode::Numpad9 => Point::new(1, -1), //move northeast
            VirtualKeyCode::Numpad3 => Point::new(1, 1),  //move southeast
            VirtualKeyCode::Numpad1 => Point::new(-1, 1), //move southwest
            _ => Point::new(0, 0),
        };

        let (player_entity, destination) = players
                .iter(ecs)
                .find_map(|(entity, pos)| Some((*entity, *pos + delta)) )
                .unwrap();

        let mut did_something = false;
        if delta.x !=0 || delta.y != 0 {

            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| {
                    **pos == destination
                })
                .for_each(|(entity, _) | {
                    hit_something = true;
                    did_something = true;

                    commands
                        .push(((), WantsToAttack{
                            attacker: player_entity,
                            victim: *entity,
                        }));
                });

            if !hit_something {
                did_something = true;
                commands
                    .push(((), WantsToMove{
                        entity: player_entity,
                        destination
                    }));
            }
        };
        if !did_something {
            if let Ok(mut health) = ecs
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current = i32::min(health.max, health.current+1);
            }
        }
        *turn_state = TurnState::PlayerTurn;
    }
}
