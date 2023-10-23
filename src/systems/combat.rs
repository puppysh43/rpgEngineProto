use std::ops::DerefMut;

use crate::prelude::*;

pub fn combat(state: &mut State, commands: &mut CommandBuffer) {
    let mut attackers = state.ecs.query::<&WantsToAttack>();

    let player_entity = state.ecs.query::<&Player>().iter().nth(0).unwrap().0; //player entity to check if the victim of an attack is the player

    let victims: Vec<(Entity, Entity)> = attackers
        .iter()
        .map(|(entity, attack)| (entity, attack.victim))
        .collect();

    victims.iter().for_each(|(message, victim)| {
        let is_player = *victim == player_entity;
        let health_entity_ref = state.ecs.entity(*victim).unwrap();
        let mut health_component_ref = health_entity_ref
            .get::<&mut Health>()
            .expect("Entity doesn't have a health component!");
        if let mut health = health_component_ref.deref_mut() {
            health.current -= 1;

            if health.current < 1 && !is_player {
                commands.despawn(*victim);
            }
        }
        commands.despawn(*message);
        commands.spawn((
            (),
            AddToLog {
                body: "An attack has occured!".to_string(),
            },
        ));
    });
}
