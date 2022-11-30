use crate::error::TicTacToeError;
use anchor_lang::prelude::*;
use num_derive::*;
use num_traits::*;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");


pub struct Game {                       // bytes
    player: [Pubkey; 2],                // 32 * 2
    turn: u8,                           // 1
    board: [[Option<Sign>; 3]; 3],      // 9 * (1+1) = 18
    state: GameState,                   // 32 + 1  why?
}

/*
The enums GameState & Sign both derive some traits. In particular 
they derive the traits AnchorSerialize and AnchorDeserialize. 
All types that are used in types that are marked with #[account] 
must implement these two traits (or be marked with #[account] themselves).
*/
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Tie,
    Won { winner: Pubkey },
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
pub enum Sign {
    X,
    O
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Tile {
    row: u8,
    column: u8,
}


impl Game {
    pub const MAXIMUM_SIZE: usize = 32*2 + 1 + 9*(1+1) + 32 + 1;

    pub fn start(&mut self, players: [Pubkey; 2]) -> Result<()> {
        require_eq!(self.turn, 0, TicTacToeError::GameAlreadyStarted);
        self.players = players;
        self.turn = 1;
        Ok(())
    }

    pub fn is_active(&self) -> bool {
        self.state == GameState::Active
    }

    /*
    This returns either 0 | 1
    If the remainder is 1, then it's an odd turn, and Player 1 is on
    If the remainder is 0, then it's an even turn, and Payer 2 is on
     */
    
    pub fn current_player_index(&self) -> usize {
        ((self.turn - 1) % 2) as usize
    }

    pub fn current_player(&self) -> Pubkey {
        self.players[self.current_player_index()]
    }

    pub fn play(&self) -> {
        //TO COME
    }

    pub fn is_winnig_trio(&self) -> {
        //TO COME
    }

    pub fn update_state(&self) -> {
        //TO COME
    }
}
