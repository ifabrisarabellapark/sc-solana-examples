use anchor_lang::prelude::*;
use instructions::*;
use state::game::Tile;

pub mod errors;
pub mod state;
pub mod instructions;


declare_id!("CzgE1QHdwsigEvPMJXtNHNgEmjKBVhZn3sPHRrU7yKga");

#[program]
pub mod tictactoe {
    use super::*;

    pub fn setup_game(ctx: Context<SetUpGame>, player_two: Pubkey) -> Result<()> {
        instructions::setup_game::setup_game(ctx, player_two)
    }

    pub fn playoff(ctx: Context<Playoff>, tile: Tile) -> Result<()> {
        instructions::play::playoff(ctx, tile)
    }
}
