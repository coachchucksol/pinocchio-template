use pinocchio::{account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError, pubkey::Pubkey, sysvars::rent::Rent, ProgramResult};
use pinocchio_system::instructions::CreateAccount;
use crate::{accounts::config::Config, utils::{load_ix_data, load_signer, load_system_account, load_system_program, DataLen}};
use pinocchio_log::log;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InitializeConfigIxData {
    pub config_bump: u8,
    pub game_fee_bps: u32,
}

impl DataLen for InitializeConfigIxData {
    const LEN: usize = core::mem::size_of::<InitializeConfigIxData>(); 
}

pub fn process_initilaize_config(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [config, base, admin, server, sysvar_rent, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let ix_data = unsafe { load_ix_data::<InitializeConfigIxData>(data)? };

    // ----------------------- CHECKS -----------------------
    load_system_program(system_program)?;
    load_system_account(config, true)?;
    load_signer(admin, true)?;

    // Check PDA is correct
    let pda = Config::create_program_address(program_id, &base.key(), ix_data.config_bump)?;
    if config.key().ne(&pda) {
        log!("Config account has an invalid key");
        return Err(ProgramError::InvalidAccountData);
    };

    // ----------------------- WORK -----------------------

    let rent = Rent::from_account_info(sysvar_rent)?;
    let bump = [ix_data.config_bump];
    let seed = [Seed::from(Config::SEED), Seed::from(base.key()), Seed::from(&bump)];
    let seeds = Signer::from(&seed);

    CreateAccount {
        from: admin,
        to: config,
        space: Config::LEN as u64,
        owner: &crate::ID,
        lamports: rent.minimum_balance(Config::LEN),
    }
    .invoke_signed(&[seeds])?;
  
    unsafe {
        Config::initialize(config, ix_data.config_bump, &base.key(), &admin.key(), &server.key(), ix_data.game_fee_bps)?;
    }

    Ok(())
}