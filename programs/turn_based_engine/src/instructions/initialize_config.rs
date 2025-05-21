use pinocchio::{account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError, pubkey::Pubkey, sysvars::rent::Rent, ProgramResult};
use pinocchio_system::instructions::CreateAccount;
use crate::{accounts::config::Config, config_seed_with_bump,  utils::{load_ix_data, load_signer, load_system_account, load_system_program, DataLen}};
use pinocchio_log::log;

use super::GameEngineInstructions;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InitializeConfigIxData {
    pub discriminator: u8,
    pub config_bump: u8,
    pub game_fee_bps: u32,
}

impl InitializeConfigIxData {
    pub fn new(
        config_bump: u8,
        game_fee_bps: u32,
    ) -> Self {
        Self { discriminator: GameEngineInstructions::InitializeConfig as u8, config_bump, game_fee_bps }
    }

    pub unsafe fn to_bytes(&self) -> &[u8] {
        unsafe { crate::utils::to_bytes::<Self>(&self) }
    }
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

    let bump_bytes = [ix_data.config_bump];
    let seed_with_bump = config_seed_with_bump!(base.key(), &bump_bytes);
    let signing_seeds = [
        Seed::from(seed_with_bump[0]), 
        Seed::from(seed_with_bump[1]), 
        Seed::from(seed_with_bump[2])
    ];
    Config::check_seeds(base.key(), ix_data.config_bump, &signing_seeds)?;
    let signer = Signer::from(&signing_seeds);

    CreateAccount {
        from: admin,
        to: config,
        space: Config::LEN as u64,
        owner: &crate::ID,
        lamports: rent.minimum_balance(Config::LEN),
    }
    .invoke_signed(&[signer])?;
  
    unsafe {
        Config::initialize(config, &base.key(), &admin.key(), &server.key(), &ix_data)?;
    }

    Ok(())
}