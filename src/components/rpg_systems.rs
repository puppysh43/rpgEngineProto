///this enum contains all the traits in the game that can be used by characters
pub enum Trait {
    NullTrait,
    Trait01,
    Trait02,
    Trait03,
}
///array that holds an entities traits in a simple tuple
pub struct Traits([Trait; 20]);

///Skills component
pub enum SkillType {
    RangedWeapons,
    MeleeWeapons,
    Athletics,
    Reflexes,
    Senses,
    Stealth,
    Lockpick,
    Technology,
    Medicine,
    Nature,
    Threaten,
    Manipulate,
    Charm,
    SocialCues,
}
pub struct Skills {
    ranged_weapons: i32,
    melee_weapons: i32,
    athletics: i32,
    reflexes: i32,
    senses: i32,
    stealth: i32,
    lockpick: i32,
    technology: i32,
    medicine: i32,
    nature: i32,
    threaten: i32,
    manipulate: i32,
    charm: i32,
    social_cues: i32,
}
impl Skills {
    // pub fn new() -> Self {}
    pub fn get_skill(&self, skill: SkillType) -> i32 {
        match skill {
            SkillType::RangedWeapons => self.ranged_weapons,
            SkillType::MeleeWeapons => self.melee_weapons,
            SkillType::Athletics => self.athletics,
            SkillType::Reflexes => self.reflexes,
            SkillType::Senses => self.senses,
            SkillType::Stealth => self.stealth,
            SkillType::Lockpick => self.lockpick,
            SkillType::Technology => self.technology,
            SkillType::Medicine => self.medicine,
            SkillType::Nature => self.nature,
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
            SkillType::Senses => {
                if self.senses + modifier < 6 && self.senses + modifier > -6 {
                    self.senses += modifier;
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
            SkillType::Nature => {
                if self.nature + modifier < 6 && self.nature + modifier > -6 {
                    self.nature += modifier;
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
