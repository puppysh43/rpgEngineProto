use crate::prelude::*;
mod melee_combat;
mod ranged_combat;

pub fn combat_systems(state: &mut State, commands: &mut CommandBuffer) {
    //have all the various combat systems in this function
    melee_combat::melee_combat(state, commands);
    commands.run_on(&mut state.ecs);
    ranged_combat::ranged_combat(state, commands);
    commands.run_on(&mut state.ecs);
}
