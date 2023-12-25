pub enum AmmoType {
    Pistol,
    Rifle,
    Shotgun,
}
pub enum ActionType {
    Manual,
    SemiAutomatic,
    SelectFire,
    FullyAutoOnly,
}
pub enum ReloadType {
    SingleLoad,
    MagazineLoad,
}

pub struct Firearm {
    ammo_type: AmmoType,
    action: ActionType,
    has_stock: bool,
    reload_type: ReloadType,
    effective_range: i32,
    current_ammo: i32,
    ammo_capacity: i32,
}
