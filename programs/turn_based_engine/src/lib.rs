#![allow(unexpected_cfgs)]

use pinocchio::{
    account_info::AccountInfo, default_panic_handler, no_allocator, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};
use pinocchio_log::log;

pub mod accounts;
pub mod instructions;
pub mod errors;
pub mod utils;

use instructions::{initialize_config::process_initilaize_config, update_config::process_update_config, GameEngineInstructions};

pinocchio_pubkey::declare_id!("Dv8yNgZsBkebdLnet7eYNBRN6XbgLNxLKLRoaXZ12jUR");
// This is the entrypoint for the program.
program_entrypoint!(process_instruction);
//Do not allocate memory.
no_allocator!();
// Use the no_std panic handler.
default_panic_handler!();



#[inline(always)]
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let discriminator = GameEngineInstructions::try_from(&instruction_data[0])?;

    match discriminator {
        GameEngineInstructions::InitializeConfig => {
            log!("Initializing server config");
            process_initilaize_config(program_id, accounts, instruction_data)
        }
        GameEngineInstructions::UpdateConfig => {
            log!("Updating server config");
            process_update_config(program_id, accounts, instruction_data)
        }
    }
}