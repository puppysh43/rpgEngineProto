//this will be the enum that tracks the ui state (in game, looking at equipped items, looking at their inventory, examining something, etc.)
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UiState {
    Default,
    ViewingInventory,
    ViewingEquipped,
    ExaminingEntity,
    InDialogue,
    ViewingJournal,
    ViewingLog,
    InteractionMenu,
}
