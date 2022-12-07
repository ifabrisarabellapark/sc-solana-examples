// Program enrypoint

use solana_program::{
    pubkey::Pubkey,
    entrypoint,
    entrypoint::ProgramResult,
    account_info::{AccountInfo},
};


// Here we are declaring a Solana Rust-based BPF program 
// entry point supported by the latest BPFLoader.
// The entrypoint must be compatible with your chosen BPFLoader
entrypoint!(process_instruct);

fn process_instruct(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    crate::processor::process_instruct(program_id, accounts, instruction_data)
}