#[error_code]
pub enum TicTacToeError {
    TileAlreadyTaken,
    TileOutOfBound,
    NotPlayersTurn,
    GameAlreadyOver,
    GameAlreadyStarted,
}