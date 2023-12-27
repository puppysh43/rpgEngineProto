use crate::prelude::*;

///processes the ranged attack MOIs to do ranged combat
pub fn ranged_combat(state: &mut State, commands: &mut CommandBuffer) {
    let mut ranged_moi_query = state.ecs.query::<&WantsToRangedAttack>();
    for (moi_id, ranged_moi) in ranged_moi_query.iter() {
        let shooter = ranged_moi.shooter;
        let target = ranged_moi.target;
        let shooter_weapon = state.ecs.query_one::<&EquippedRangedWeapon>(shooter).expect("Somehow an entity was able to make a ranged attack MOI w/out an equipped ranged weapon");
        let attack_type = ranged_moi.attack_type;
        match attack_type {
            RangedAttackType::SingleShot => {
                //filler
            }
            RangedAttackType::TwoRoundBurst => {
                //filler
            }
            RangedAttackType::ThreeRoundBurst => {
                //filler
            }
            RangedAttackType::FullAutoFire => {
                //filler
            }
        }
        commands.despawn(moi_id);
    }
}
