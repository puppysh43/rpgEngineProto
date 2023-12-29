use crate::prelude::*;

///processes the ranged attack MOIs to do ranged combat
pub fn ranged_combat(state: &mut State, commands: &mut CommandBuffer) {
    let mut ranged_moi_query = state.ecs.query::<&WantsToRangedAttack>();
    for (moi_id, ranged_moi) in ranged_moi_query.iter() {
        let shooter = ranged_moi.shooter;
        let target = ranged_moi.target;
        let shooter_weapon = ranged_moi.shooter_weapon;
        let attack_type = ranged_moi.attack_type;
        match attack_type {
            RangedAttackType::SingleShot => {
                //filler
                //calculate the effective ranged modifier by checking the weapon and comparing it to the distance from the moi
                //check to see if target is in cover (if in_cover is okay then see what kind of cover and apply the difference)
                //will need to implement penetratable cover and map tile health etc but will do that LATER
                //have the victim do the reflexes roll to see if
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
