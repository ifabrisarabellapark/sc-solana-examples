use anchor_lang::prelude::*;
use instructions::*;
use state::game::Tile;

pub mod errors;
pub mod state;
pub mod instructions;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod tictactoe {
    use super::*;

    pub fn setup_game() -> {
        //initiat game
    }

    pub fn play() -> {
        //establish ruleset
    }
}
