use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    msg,
    account_info::{AccountInfo, next_account_info},
    pubkey::Pubkey,
    rent::Rent,
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program::invoke_signed,
    system_instruction,
    sysvar::Sysvar,
};

entrypoint!(process_instruction);

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct EcoBalance {
    is_initialized: bool,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8], 
) -> ProgramResult {
    const ACCOUNT_DATA_LEN: usize = 1;

    let acc_iter = &mut accounts.iter();
    let funding_account = next_account_info(acc_iter)?;
    let pda_account = next_account_info(acc_iter)?;
    let system_program = next_account_info(acc_iter)?;

    // get PDA bumb from instructions
    let (pda_bump, _) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    // check whether PDAs match up
    let signers_seeds: &[&[u8]; 3] = &[
        b"BalloonBox-",
        b"escrow",
        &[*pda_bump],
    ];

    // TO DO: use find_program_address() fn instead of create_..() fn
    // to generate PDA account
    let pda = Pubkey::create_program_address(signers_seeds, program_id)?;

    if pda.ne(&pda_account.key) {
        return Err(ProgramError::InvalidAccountData);
    }

    let lamports_required = Rent::get()?.minimum_balance(ACCOUNT_DATA_LEN);
    let create_pda_account_ix = system_instruction::create_account(
        &funding_account.key,
        &pda_account.key,
        lamports_required,
        ACCOUNT_DATA_LEN.try_into().unwrap(),
        &program_id,
    );

    invoke_signed(
        &create_pda_account_ix,
        &[funding_account.clone(), pda_account.clone(), system_program.clone(),],
        &[signers_seeds],
    )?;

    let mut pda_account_state = EcoBalance::try_from_slice(&pda_account.data.borrow())?;
    pda_account_state.is_initialized = true;
    pda_account_state.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;

    msg!("PDA pubkey = {}", pda.to_string());
    msg!("PDA seeds = {:?}", signers_seeds);
    msg!("PDA bump = {}", pda_bump.to_string());
    Ok(())
}