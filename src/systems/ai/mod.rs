use crate::prelude::*;

mod chasing;
mod random_move;
///This function contains all the AI systems used by various NPCs
pub fn ai_systems(state: &mut State, commands: &mut CommandBuffer) {
    random_move::random_move(state, commands); //WORKING (?)
    commands.run_on(&mut state.ecs);
    chasing::chasing(state, commands); //WORKING (?)
    commands.run_on(&mut state.ecs);
}
