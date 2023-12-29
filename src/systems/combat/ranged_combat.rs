use std::ops::RangeBounds;

use crate::prelude::*;

///processes the ranged attack MOIs to do ranged combat
pub fn ranged_combat(state: &mut State, commands: &mut CommandBuffer) {
    let mut ranged_moi_query = state.ecs.query::<&WantsToRangedAttack>();
    for (moi_id, ranged_moi) in ranged_moi_query.iter() {
        let (
            shooter,
            attack_type,
            shooter_weapon,
            shooter_skills,
            target,
            target_skills,
            target_armor,
            distance,
            is_target_ducking,
            is_in_cover,
        ) = (
            ranged_moi.shooter,
            ranged_moi.attack_type,
            ranged_moi.shooter_weapon.clone(),
            ranged_moi.shooter_skills,
            ranged_moi.target,
            ranged_moi.target_skills,
            ranged_moi.target_armor,
            ranged_moi.distance,
            ranged_moi.is_target_ducking,
            ranged_moi.is_in_cover,
        );
        match attack_type {
            RangedAttackType::SingleShot => {
                //calculate the effective ranged modifier by checking the weapon and comparing it to the distance from the moi
                //check to see if target is in cover (if in_cover is okay then see what kind of cover and apply the difference)
                //will need to implement penetratable cover and map tile health etc but will do that LATER
                //have the victim do the reflexes roll to see if they duck and if they do apply the duck bonus
                //allow player to decide if they want to autoduck or not
                //do a firearm skill check
                let (eff_range_mod, cover_mod, ducking_mod) = (
                    get_eff_range_mod(shooter_weapon.clone(), distance),
                    get_cover_mod(is_in_cover),
                    get_ducking_mod(is_target_ducking),
                );
                match shooter_skills.skillcheck(
                    SkillType::RangedWeapons,
                    eff_range_mod + cover_mod + ducking_mod,
                ) {
                    RollResult::FullSuccess => {
                        //check target armor to decide how much damage to deal
                    }
                    RollResult::PartialSuccess => {
                        //roll on the woo table!
                    }
                    RollResult::Failure => {
                        //roll on the woo table with a -2 modifier
                    }
                }
            }
            RangedAttackType::TwoRoundBurst => {
                //
            }
            RangedAttackType::ThreeRoundBurst => {
                //filler
                let eff_range_mod = get_eff_range_mod(shooter_weapon.clone(), distance);
            }
            RangedAttackType::FullAutoFire => {
                //filler
                let eff_range_mod = get_eff_range_mod(shooter_weapon.clone(), distance);
            }
        }
        commands.despawn(moi_id);
    }
}

fn get_stabilization_mod(
    shooter_weapon: Firearm,
    shooter_skills: Skills,
    attack_type: RangedAttackType,
) -> i32 {
    let rng = RandomNumberGenerator::new();
    match attack_type {
        RangedAttackType::SingleShot
        | RangedAttackType::TwoRoundBurst
        | RangedAttackType::ThreeRoundBurst => {
            if !shooter_weapon.has_stock {
                //if the weapon doesn't have a stock they need to roll for stabilization
                if shooter_skills.get_skill(SkillType::RangedWeapons)
                    > shooter_skills.get_skill(SkillType::Athletics)
                {
                    //if the shooter's firearms skill is higher than their athletics use that for stabilization
                    match shooter_skills.skillcheck(SkillType::RangedWeapons, 0) {
                        RollResult::FullSuccess => return 0,
                        RollResult::PartialSuccess => return -1,
                        RollResult::Failure => return -2,
                    }
                } else {
                    //but if the athletics is higher use that for the stabilization roll!
                    match shooter_skills.skillcheck(SkillType::Athletics, 0) {
                        RollResult::FullSuccess => return 0,
                        RollResult::PartialSuccess => return -1,
                        RollResult::Failure => return -2,
                    }
                }
            } else {
                //if it has a stock they don't need to so there's no modifier!
                return 0;
            }
        }
        RangedAttackType::FullAutoFire => {
            //if the shooter is making a full auto attack they need to roll a stabilization no matter what
            if shooter_skills.get_skill(SkillType::RangedWeapons)
                > shooter_skills.get_skill(SkillType::Athletics)
            {
                //if the shooter's firearms skill is higher than their athletics use that for stabilization
                match shooter_skills.skillcheck(SkillType::RangedWeapons, 0) {
                    RollResult::FullSuccess => return 0,
                    RollResult::PartialSuccess => return -1,
                    RollResult::Failure => return -2,
                }
            } else {
                //but if the athletics is higher use that for the stabilization roll!
                match shooter_skills.skillcheck(SkillType::Athletics, 0) {
                    RollResult::FullSuccess => return 0,
                    RollResult::PartialSuccess => return -1,
                    RollResult::Failure => return -2,
                }
            }
        }
    }
}

fn get_eff_range_mod(firearm: Firearm, distance: i32) -> i32 {
    let effective_range = firearm.effective_range;
    let is_long_gun = firearm.ammo_type == AmmoType::Rifle;
    if distance <= effective_range {
        //if the distance is within effective range there's no modifiers
        if distance <= (effective_range as f32 * 0.20) as i32 {
            //if the target is within the bottom 20% of the effective rnage there's a +1
            if distance < 2 && is_long_gun {
                //if a character is using a long gun to shoot something in arm's reach there's a -2 mod
                return -2;
            }
            return 1;
        }
        return 0;
    } else {
        //if the distance is greater than the effective range divide the distance by the range for the modifier
        return distance / effective_range;
    }
}

fn get_cover_mod(is_in_cover: Option<CoverType>) -> i32 {
    //if the target is in cover check what kind
    if is_in_cover.is_some() {
        match is_in_cover.unwrap() {
            CoverType::Half => return -1,
            CoverType::Full => return -2,
        }
    } else {
        //if they're not in cover there's no modifier
        return 0;
    }
}

fn get_ducking_mod(is_target_ducking: bool) -> i32 {
    if is_target_ducking {
        return -2;
    } else {
        return 0;
    }
}
