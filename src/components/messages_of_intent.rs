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
pub struct WantsToRangedAttack {
    pub shooter: Entity,
    pub target: Entity,
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
///
#[derive(Clone, Debug, PartialEq)]
pub struct WantsToChangeMap {
    pub pos: Point,
    pub entity: Entity,
    pub cardinal_direction: CardinalDirection,
    pub map_pos: Point3D,
    pub current_localmap: LocalMapID,
}
