// Export object types from a few Rust crates
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo},
    msg,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Greetings {
    pub counter: u32,
}

// Here we are declaring a Solana Rust-based BPF program 
// entry point supported by the latest BPFLoader.
// The entrypoint must be compatible with your chosen BPFLoader
entrypoint!(helloworld);


// Implement program entrypoint
pub fn helloworld(
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Helloworld entrypoint");

    let mut greet = Greetings { counter: 0u32 };
    greet.counter += 1;

    msg!("Greeted {} time(s)!", greet.counter);
    Ok(())
}

