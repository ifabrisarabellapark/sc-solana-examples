use anchor_lang::prelude::*;
use crate::state::game::*;
use crate::errors::TicTacToeError;

pub fn playoff(ctx: Context<Playoff>, tile: Tile) -> Result<()> {
    let mygame = &mut ctx.accounts.game;

    require_keys_eq!(
        mygame.current_player(),
        ctx.accounts.player.key(),
        TicTacToeError::NotPlayersTurn
    );

    mygame.play(&tile)
}

#[derive(Accounts)]
pub struct Playoff<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    pub player: Signer<'info>,
}