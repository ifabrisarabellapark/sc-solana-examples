use anchor_lang::prelude::*;
use crate::state::game::*;

pub fn setup_game(ctx: Context<SetUpGame>, player_two: Pubkey) -> Result<()> {
    ctx.accounts.game.start([ctx.accounts.player_one.key(), player_two])
}

#[derive(Accounts)]
pub struct SetUpGame<'info> {
    #[account(init, payer = player_one, space = 8 + Game::MAXIMUM_SIZE)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player_one: Signer<'info>,
    pub system_program: Program<'info, System>,
}