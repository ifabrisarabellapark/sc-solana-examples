use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::instruction::transfer;

entrypoint!(process_instruction);


// Accounts required
/// 1. [writable] Source Token Account
/// 2. [writable] Destination Token Account
/// 3. [signer] Source Token Account holder's PubKey
/// 4. [] Token Program
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // Accounts required for token transfer

    // 1. Associated Token account we hold
    let source_token_account = next_account_info(accounts_iter)?;
    // 2. Our wallet address
    let source_token_account_holder = next_account_info(accounts_iter)?;
    // 3. Token Program
    let token_program = next_account_info(accounts_iter)?;
    // 4. Associated Token account to send to
    let destination_token_account = next_account_info(accounts_iter)?;

    // Parsing the token transfer amount from instruction data
    let amount = instruction_data
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(ProgramError::InvalidAccountData)?;

    msg!(
        "Transferring {} token(s) from {} to {}",
        amount,
        source_token_account.key.to_string(),
        destination_token_account.key.to_string()
    );

    // Call the `transfer` function from the Solana Token Program
    let transfer_tokens_instruction = transfer(
        &token_program.key,
        &source_token_account.key,
        &destination_token_account.key,
        &source_token_account_holder.key,
        &[&source_token_account_holder.key],
        amount*u64::pow(10, 9),
    )?;

    let required_accounts_for_transfer = [
        source_token_account.clone(),
        destination_token_account.clone(),
        source_token_account_holder.clone(),
    ];

    // Cross contract invocation
    // Passing the TransactionInstruction to send
    invoke(
        &transfer_tokens_instruction,
        &required_accounts_for_transfer,
    )?;

    msg!("SPL-token transfer successful");
    Ok(())
}