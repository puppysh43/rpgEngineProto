#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    AwaitingInput,
    PcTurn,
    NpcTurn,
    GameOver,
    EndingSlides,
    MainMenu,
}
