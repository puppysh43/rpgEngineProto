#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ControlState {
    Default, //Baseline control state for general world movement
    // Melee,       //control state for when selecting the direction to melee attack in
    // Aiming,      //control state for when the player is moving a reticule to shoot a gun
    Looking, //ctrl state for when the player is moving a reticule to examine something
    //Inventory //control state for when the player has opened up the inventory menu
    // Dropping, //ctrl state for when the player is selecting what item to drop (using alphanumeric keys)
    // Interacting, //ctrl state for when the player needs to select what to interact with
    // Throwing, //ctrl state for when the player is selecting where they want to throw something.
    // UsingSkill, //ctrl state for when the player is selecting a skill to use
    // MeleeSkill, //ctrl state for when the player is selecting a direction to use the skill
    // RangedSkill, //ctrl state for selecting a ranged point to use the skill (may not need this one)
    // PickingUp, //ctrl state for when deciding what to pick up out of multiple items
    // SelectingItem,
    // TargetingItem,
    ExaminingEntity,
}
