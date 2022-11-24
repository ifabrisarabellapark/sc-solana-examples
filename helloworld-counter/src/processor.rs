// Instructor processor

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    log::{sol_log_compute_units, sol_log_params},
    msg,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Greetings {
    pub counter: u32,
}

/*
    Implement program entrypoint. The entrypoint always calls 
    the instructor processor function (i.e. 'process_instruct')
    with these 3 params: program_id, accounts, instruction_data.
    You must declare ALL 3 params even if unused.
    Add an _ in front of the variable name, if unused.
 */ 
pub fn process_instruct(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    // iterate through accounts and get account to say hello to    
    let acc_iter = &mut accounts.iter();
    let account = next_account_info(acc_iter)?;

    // deserialize and borrow a slice of bytes, namely the greeting count, 
    // next update the count and serialize it back to chain
    let mut greet = Greetings::try_from_slice(&account.data.borrow())?;
    greet.counter += 1;
    greet.serialize(&mut &mut account.data.borrow_mut()[..])?;

    /*
        LOG EVENTS
     */
    // log a public key
    msg!("The PROGRAM ID is = ");
    program_id.log();
    // log a formatted string, beware: expensive!
    msg!("Greeted {} time(s)!", greet.counter);
    // log all the program's input parameters
    sol_log_params(accounts, instruction_data);
    // log the number of compute units remaining to consume
    sol_log_compute_units();


    Ok(())
}




/*
 * the rest of this file sets up unit tests
 * execute them running the command:
 * cargo test --package event_emitter -- --nocapture
 * Note: 'event_emitter' comes from Cargo.toml's 'name' key
 */
#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use solana_program::clock::Epoch;
    
    #[test]
    fn count_greeter() {
        let key = Pubkey::default();
        let mut lamports = 0u64;
        let mut data = vec![0; mem::size_of::<u32>()];
        let program_id = Pubkey::default();
        let instruction_data: Vec<u8> = Vec::new();
        let accounts = vec![AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &key,
            false,
            Epoch::default(),
        )];


        // use .unwrap() for implicit handling of an Option<T>
        process_instruct(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(Greetings::try_from_slice(&accounts[0].data.borrow()).unwrap().counter,1);

        // greet 3 times
        process_instruct(&program_id, &accounts, &instruction_data).unwrap();
        process_instruct(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(Greetings::try_from_slice(&accounts[0].data.borrow()).unwrap().counter,3);
    }
}