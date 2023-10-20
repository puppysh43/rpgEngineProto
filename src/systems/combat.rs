use crate::prelude::*;

pub fn combat(state: &mut State) {
    let mut commands = CommandBuffer::new();
    let mut attackers = state.ecs.query::<&WantsToAttack>();
    let victims: Vec<(Entity, Entity)> = attackers
        .iter()
        .map(|(entity, attack)| (entity, attack.victim))
        .collect();

    victims.iter().for_each(|(message, victim)| {
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= 1;
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
        }
        commands.remove(*message);
        commands.push((
            (),
            AddToLog {
                body: "An attack has occured!".to_string(),
            },
        ));
    });
}
