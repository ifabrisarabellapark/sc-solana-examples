use crate::errors::TicTacToeError;
use anchor_lang::prelude::*;
use num_derive::*;
use num_traits::*;


declare_id!("CzgE1QHdwsigEvPMJXtNHNgEmjKBVhZn3sPHRrU7yKga");


#[account]
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

#[derive(
    AnchorSerialize, AnchorDeserialize, FromPrimitive, ToPrimitive, Copy, Clone, PartialEq, Eq,
)]
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
        self.player = players;
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
        self.player[self.current_player_index()]
    }

    pub fn is_winning_trio(&self, trio: [(usize, usize); 3]) -> bool {
        let [first, second, third] = trio;
        self.board[first.0][first.1].is_some() 
            && self.board[first.0][first.1] == self.board[second.0][second.1]
            && self.board[first.0][first.1] == self.board[third.0][third.1]
    }

    pub fn update_state(&mut self) {
        for i in 0..=2 {
            // three in a row OR column
            if self.is_winning_trio([(i, 0), (i, 1), (i, 2)]) 
                || self.is_winning_trio([(0, i), (1, i), (2, i)]){
                self.state = GameState::Won {
                    winner: self.current_player(),
                };
                return;
            };
        };

        // three in a diagonal
        if self.is_winning_trio([(0, 0), (1, 1), (2, 2)])
            || self.is_winning_trio([(0, 2), (1, 1), (2, 0)]) {
            self.state = GameState::Won {
                winner: self.current_player(),
        };
        return;
    };

        // free tiles still
        for x in 0..=2 {
            for y in 0..=2 {
                if self.board[x as usize][y as usize].is_none() {
                    return;
                }
            }
        }
 
        // tie
        self.state = GameState::Tie;
    }



    pub fn play(&mut self, tile: &Tile) -> Result<()> {
        require!(self.is_active(), TicTacToeError::GameAlreadyOver);
        /*
        'tile' here is an example of binding within a match pattern
        Create a temporary variable within a match construct,
        and bind the result of the pattern to it
         */
        match tile {
            tile @ Tile {
                row: 0..=2,
                column: 0..=2,
            } => match self.board[tile.row as usize][tile.column as usize] {
                    Some(_) => return Err(TicTacToeError::TileAlreadyTaken.into()),
                    None => {
                        self.board[tile.row as usize][tile.column as usize] = 
                        Some(Sign::from_usize(self.current_player_index()).unwrap());
                    },
                },
            _ => return Err(TicTacToeError::TileOutOfBound.into()),
        } 

        self.update_state();

        if self.is_active() {
            self.turn += 1;
        };

        Ok(())
    }
}
