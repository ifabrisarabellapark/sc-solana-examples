use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program::{invoke},
    program_error::ProgramError,
    system_instruction,
    msg,
};
use std::convert::TryInto;


entrypoint!(process_instruction);


// on-chain program: instruction function
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {

    // read accounts
    let acc_iter = &mut accounts.iter();
    let payer = next_account_info(acc_iter)?;
    let payee = next_account_info(acc_iter)?;  

    // deserialized byte array (8 bytes) into an integer
    let amount = input
        .get(..8)
        .and_then(|slice| slice.try_into().ok()) //lambda: turn slice to int
        .map(u64::from_le_bytes)
        .ok_or(ProgramError::InvalidInstructionData)?;

    // log events
    msg!("Request to transfer {:?} SOL from {:?} to {:?}",
    amount, payer.key, payee.key);
    msg!("Transfer in progress...");

    // transfer from PAYER to PAYEE an amount of SOL
    invoke(
        &system_instruction::transfer(payer.key, payee.key, amount),
        &[payer.clone(), payee.clone()],
    )?;

    msg!("Transfer succeeded!");
    Ok(())
}