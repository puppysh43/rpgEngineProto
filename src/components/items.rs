use crate::prelude::*;
//GENERIC ITEM COMPONENTS
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Item;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ProvidesHealing {
    pub amount: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Carried(pub Entity);

#[derive(Clone, Debug, PartialEq)]
pub struct Key(pub String);
//ALL OF THESE COMPONENTS ARE ABOUT THE GUN SYSTEM
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AmmoType {
    Pistol,
    Rifle,
    Shotgun,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ActionType {
    Manual,
    SemiAutomatic,
    SelectFire,
    FullyAutoOnly,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ReloadType {
    SingleLoad,
    MagazineLoad,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Firearm {
    pub ammo_type: AmmoType,
    pub action: ActionType,
    pub has_stock: bool,
    pub reload_type: ReloadType,
    pub effective_range: i32,
    pub current_ammo: i32,
    pub ammo_capacity: i32,
}

impl Firearm {
    pub fn new(
        ammo_type: AmmoType,
        action: ActionType,
        has_stock: bool,
        reload_type: ReloadType,
        effective_range: i32,
        current_ammo: i32,
        ammo_capacity: i32,
    ) -> Self {
        Self {
            ammo_type,
            action,
            has_stock,
            reload_type,
            effective_range,
            current_ammo,
            ammo_capacity,
        }
    }
    pub fn manual_rifle() -> Self {
        Self {
            ammo_type: AmmoType::Rifle,
            action: ActionType::Manual,
            has_stock: true,
            reload_type: ReloadType::MagazineLoad,
            effective_range: 50,
            current_ammo: 5,
            ammo_capacity: 5,
        }
    }
    pub fn single_action_revolver() -> Self {
        Self {
            ammo_type: AmmoType::Pistol,
            action: ActionType::Manual,
            has_stock: false,
            reload_type: ReloadType::SingleLoad,
            effective_range: 20,
            current_ammo: 6,
            ammo_capacity: 6,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ArmorType {
    None,
    Light,
    Medium,
    Heavy,
}
