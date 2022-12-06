
// import all components of the Rust crate called anchor_lang
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    entrypoint::ProgramResult,
};

// Your program's id
// Your full program keypair (pubkey & privatekey) is now in `target/deploy`
// The public key of your keypair IS the program id
// declare_id!("FzJeAJtHKAY3LJoDVpZjmbrmsMGp7wA8AhafHRugDfLZ"); // testing program id
// declare_id!("7Gjakg7gAYgCRr7p5RvuGoS1G5EMQxu9U61Swokse5bj"); // deployed program id
declare_id!("72tyNt2vq4ySQsBS5GXxiPhJh3pSEE68FNcrm3vasxKf");

#[program]
pub mod casio {
    use super::*;

    pub fn create(ctx: Context<Create>, init_message: String) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }

    pub fn add(ctx: Context<Addition>, n1: i64, n2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = n1 + n2;
        Ok(())
    }
    
    pub fn multiply(ctx: Context<Multiplication>, n1: i64, n2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = n1 * n2;
        Ok(())
    }
    
    pub fn subtract(ctx: Context<Subtraction>, n1: i64, n2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = n1 - n2;
        Ok(())
    }
    
    pub fn divide(ctx: Context<Division>, n1: i64, n2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = n1 / n2;
        calculator.remainder = n1 % n2;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer=user, space=8+64+64+64+64)]
    pub calculator: Account<'info, Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Calculator {
    pub greeting: String,
    pub result: i64,
    pub remainder: i64,
}

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Multiplication<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Subtraction<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Division<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}
