use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program::{invoke}, //invoke_signed
    program_error::ProgramError,
    system_instruction,
    msg,
};
// use spl_token;
use spl_associated_token_account;
use std::convert::TryInto;


entrypoint!(process_instruction);


// on-chain program instruction function.
// functions arguments are just the Solana boilerplate ones
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {

    // read accounts
    let acc_iter = &mut accounts.iter();

    // 1. Keypair - the user requesting ECOV
    let user = next_account_info(acc_iter)?;
    // 2. Keypair - SPL-token account holder
    let vault = next_account_info(acc_iter)?;
    // 3. Pubkey - ECOV recipient: user's ECOV Associated Token Account (ATA)
    let user_ata = next_account_info(acc_iter)?;
    // 4. Pubkey - ECOV sender: SPL-token account (ATA)
    let vault_ata = next_account_info(acc_iter)?;
    // 5. SOL liquidity Cache which BBox uses to cash-in user's SOL payments
    let bbox_sol_payee = next_account_info(acc_iter)?;
    // 6. Solana TokenProgram
    let token_program = next_account_info(acc_iter)?;
    // 7. Solana Systemprogram 
    let system_program = next_account_info(acc_iter)?;


    // deserialized byte array (8 bytes) into an integer
    let amount = input
        .get(..8)
        .and_then(|slice| slice.try_into().ok()) //lambda: turn slice to int
        .map(u64::from_le_bytes)
        .ok_or(ProgramError::InvalidInstructionData)?;

    msg!("Request to recieve {:?} ECOV from user {:?}",
    amount, user.key);
    msg!("SOL Transfer in progress...");

    // #1 Cross program invocation:
    // transfer SOL from USER to PAYEE.
    // The transaction signer is the USER.
    invoke(
        &system_instruction::transfer(
            user.key,
            bbox_sol_payee.key,
            amount*u64::pow(10, 9)
        ),
        &[
            user.clone(),
            bbox_sol_payee.clone()
        ],
    )?;

    msg!("SOL transfer succeeded!");
    msg!(
        "Transfer {} SPL-token from {:?} to {:?}",
        amount, vault_ata.key, user_ata.key
    );


    // #2 Cross program invocation:
    // transfer ECOV from VAULT_ATA to USER_ATA.
    // The trasaction signer is the VAULT.
    invoke(
        //instructions
        &spl_token::instruction::transfer(
            token_program.key,
            vault_ata.key,
            user_ata.key,
            vault.key,
            &[&vault.key],
            amount*u64::pow(10, 9),
        )?,
        //accounts
        &[
            vault_ata.clone(),
            user_ata.clone(),
            vault.clone(),
            token_program.clone(),
        ],
    )?;
    msg!("SPL-token transfer succeeded!");

    // finally
    Ok(())
}