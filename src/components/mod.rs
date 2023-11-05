pub use crate::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
///tagging component that marks something as an effect to be rendered over the other game objects
pub struct Effect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

//Messages of Intent START
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AddToLog {
    pub body: String,
}
//Messages of Intent END

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}
//description related components
#[derive(Clone, PartialEq)]
///Component used to give an entity a name that'll
pub struct Name(pub String);

#[derive(Clone, Debug, PartialEq)]
///Component used to give an entity a short (sub 35 character) description
///that will be printed to the log when they're examined
pub struct ShortDescription(pub String);

#[derive(Clone, Debug, PartialEq)]
///Component used to give an entity a long description that will be displayed on its own screen.
pub struct LongDescription(pub String);

#[derive(Clone, Copy, Debug)]
///Component used to tag an entity as being examined by the player so the appropriate information can
///later be read and displayed by the UI
pub struct Examining;
#[derive(Clone, Copy, Debug, PartialEq)]
///Tagging component that marks something as a reticule for selecting stuff in game
pub struct Reticule;

//FOV Related Components
#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Location(pub MapID);
