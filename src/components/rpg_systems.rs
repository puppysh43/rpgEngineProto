use crate::prelude::*;
///this enum contains all the traits in the game that can be used by characters
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Trait {
    NullTrait,
    Trait01,
    Trait02,
    Trait03,
}
#[derive(Copy, Clone, Debug, PartialEq)]
///array that holds an entities traits in a simple tuple
pub struct Traits([Trait; 20]);

#[derive(Clone, Debug, PartialEq)]
pub struct EquippedRangedWeapon(Option<Firearm>);

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CoverType {
    Half,
    Full,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IsDucking(bool);

///Skills component
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SkillType {
    RangedWeapons,
    MeleeWeapons,
    Athletics,
    Reflexes,
    Perception,
    Stealth,
    Lockpick,
    Technology,
    Medicine,
    PlanetSense,
    Threaten,
    Manipulate,
    Charm,
    SocialCues,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Skills {
    ranged_weapons: i32,
    melee_weapons: i32,
    athletics: i32,
    reflexes: i32,
    perception: i32,
    stealth: i32,
    lockpick: i32,
    technology: i32,
    medicine: i32,
    planetsense: i32,
    threaten: i32,
    manipulate: i32,
    charm: i32,
    social_cues: i32,
}
impl Skills {
    pub fn new(
        ranged_weapons: i32,
        melee_weapons: i32,
        athletics: i32,
        reflexes: i32,
        perception: i32,
        stealth: i32,
        lockpick: i32,
        technology: i32,
        medicine: i32,
        planetsense: i32,
        threaten: i32,
        manipulate: i32,
        charm: i32,
        social_cues: i32,
    ) -> Self {
        Self {
            ranged_weapons,
            melee_weapons,
            athletics,
            reflexes,
            perception,
            stealth,
            lockpick,
            technology,
            medicine,
            planetsense,
            threaten,
            manipulate,
            charm,
            social_cues,
        }
    }
    pub fn new_blank() -> Self {
        Self {
            ranged_weapons: 0,
            melee_weapons: 0,
            athletics: 0,
            reflexes: 0,
            perception: 0,
            stealth: 0,
            lockpick: 0,
            technology: 0,
            medicine: 0,
            planetsense: 0,
            threaten: 0,
            manipulate: 0,
            charm: 0,
            social_cues: 0,
        }
    }
    pub fn get_skill(&self, skill: SkillType) -> i32 {
        match skill {
            SkillType::RangedWeapons => self.ranged_weapons,
            SkillType::MeleeWeapons => self.melee_weapons,
            SkillType::Athletics => self.athletics,
            SkillType::Reflexes => self.reflexes,
            SkillType::Perception => self.perception,
            SkillType::Stealth => self.stealth,
            SkillType::Lockpick => self.lockpick,
            SkillType::Technology => self.technology,
            SkillType::Medicine => self.medicine,
            SkillType::PlanetSense => self.planetsense,
            SkillType::Threaten => self.threaten,
            SkillType::Manipulate => self.manipulate,
            SkillType::Charm => self.charm,
            SkillType::SocialCues => self.social_cues,
            _ => 0,
        }
    }
    pub fn change_skill(&mut self, skill: SkillType, modifier: i32) {
        match skill {
            SkillType::RangedWeapons => {
                if self.ranged_weapons + modifier < 6 && self.ranged_weapons + modifier > -6 {
                    self.ranged_weapons += modifier;
                }
            }
            SkillType::MeleeWeapons => {
                if self.melee_weapons + modifier < 6 && self.melee_weapons + modifier > -6 {
                    self.melee_weapons += modifier;
                }
            }
            SkillType::Athletics => {
                if self.athletics + modifier < 6 && self.athletics + modifier > -6 {
                    self.athletics += modifier;
                }
            }
            SkillType::Reflexes => {
                if self.reflexes + modifier < 6 && self.reflexes + modifier > -6 {
                    self.reflexes += modifier;
                }
            }
            SkillType::Perception => {
                if self.perception + modifier < 6 && self.perception + modifier > -6 {
                    self.perception += modifier;
                }
            }
            SkillType::Stealth => {
                if self.stealth + modifier < 6 && self.stealth + modifier > -6 {
                    self.stealth += modifier;
                }
            }
            SkillType::Lockpick => {
                if self.lockpick + modifier < 6 && self.lockpick + modifier > -6 {
                    self.lockpick += modifier;
                }
            }
            SkillType::Technology => {
                if self.technology + modifier < 6 && self.technology + modifier > -6 {
                    self.technology += modifier;
                }
            }
            SkillType::Medicine => {
                if self.medicine + modifier < 6 && self.medicine + modifier > -6 {
                    self.medicine += modifier;
                }
            }
            SkillType::PlanetSense => {
                if self.planetsense + modifier < 6 && self.planetsense + modifier > -6 {
                    self.planetsense += modifier;
                }
            }
            SkillType::Threaten => {
                if self.threaten + modifier < 6 && self.threaten + modifier > -6 {
                    self.threaten += modifier;
                }
            }
            SkillType::Manipulate => {
                if self.manipulate + modifier < 6 && self.manipulate + modifier > -6 {
                    self.manipulate += modifier;
                }
            }
            SkillType::Charm => {
                if self.charm + modifier < 6 && self.charm + modifier > -6 {
                    self.charm += modifier;
                }
            }
            SkillType::SocialCues => {
                if self.social_cues + modifier < 6 && self.social_cues + modifier > -6 {
                    self.social_cues += modifier;
                }
            }
        };
    }
}
