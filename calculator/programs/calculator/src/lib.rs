use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn create(ctx: Context<Create>, init_message: String) -> ProgramResult {}

    pub fn add(ctx: Context<Addition>, n1: i64, n2: i64) -> ProgreamResult {}
    
    pub fn multiply(ctx: Context<Multiplication>, n1: i64, n2: i64) -> ProgreamResult {}
    
    pub fn subtract(ctx: Context<Subtraction>, n1: i64, n2: i64) -> ProgreamResult {}
    
    pub fn divide(ctx: Context<Division>, n1: i64, n2: i64) -> ProgreamResult {}
}

#[derive(Accounts)]
pub struct Initialize {}
