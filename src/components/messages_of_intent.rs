use crate::prelude::*;

//these are all of the components used as messages of intent
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RangedAttackType {
    SingleShot,
    TwoRoundBurst,
    ThreeRoundBurst,
    FullAutoFire,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WantsToRangedAttack {
    pub shooter: Entity,
    pub attack_type: RangedAttackType,
    pub shooter_weapon: Firearm,
    pub shooter_skills: Skills,
    pub target: Entity,
    pub target_skills: Skills,
    pub target_armor: ArmorType,
    pub is_target_ducking: bool,
    pub is_in_cover: Option<CoverType>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AddToLog {
    pub body: String,
}
///message of intent for when something on the worldmap wants to enter a location
#[derive(Clone, Debug, PartialEq)]
pub struct WantsToEnterLocation {
    pub pos: Point,
    pub entity: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
///Attach this tag to an entity and it will be unable to do ANYTHING for the specified number of rounds
pub struct UnableToAct(i32);
///
#[derive(Clone, Debug, PartialEq)]
pub struct WantsToChangeMap {
    pub pos: Point,
    pub entity: Entity,
    pub cardinal_direction: CardinalDirection,
    pub map_pos: Point3D,
    pub current_localmap: LocalMapID,
}
