use anchor_lang::error_code;

#[error_code]
pub enum TicTacToeError {
    TileAlreadyTaken,
    TileOutOfBound,
    NotPlayersTurn,
    GameAlreadyOver,
    GameAlreadyStarted,
}