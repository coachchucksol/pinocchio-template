
use pinocchio::{account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError, pubkey::{self, Pubkey}};
use pinocchio_log::log;

use crate::{utils::{load_account, load_account_mut_unchecked, DataLen, Initialized}, accounts::GameEngineDiscriminator};

/// The Counter account structure
#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
pub struct Config {
    discriminator: u8,
    bump: u8,
    base: Pubkey,
    admin: Pubkey,
    server: Pubkey,
    game_fee_bps: u32,
}

impl DataLen for Config {
    const LEN: usize = core::mem::size_of::<Config>();
}

impl Initialized for Config {
    fn is_initialized(&self) -> bool {
        self.discriminator == GameEngineDiscriminator::Config as u8
    }
}

impl Config {
    // ----------------------- ACCOUNT CHECKS ---------------------------
    pub const SEED: &[u8] = b"CONFIG";


    pub fn create_program_address<'a>(
        program_id: &Pubkey,
        base: &Pubkey,
        bump: u8,
    ) -> Result<Pubkey, ProgramError> {
        let seed_with_bump = [Self::SEED, base.as_ref(), &[bump]];
        let pda = pubkey::create_program_address(&seed_with_bump, program_id)?;

        Ok(pda)
    }

    pub fn load(
        program_id: &Pubkey,
        account_info: &AccountInfo,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        let account_owner = unsafe { account_info.owner() };
        if account_owner.ne(program_id) {
            log!("Config account has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }

        if expect_writable && !account_info.is_writable() {
            log!("Config account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }

        let account = unsafe { 
            let data = account_info.borrow_data_unchecked();
            let result = load_account::<Config>(data);

            if let Err(error) = result {
                log!("Config account could not be deseralized");
                return Err(error);
            }
            result?
        };

        let account_key = Self::create_program_address(program_id, &account.base, account.bump)?;
        if account_info.key().ne(&account_key) {
            log!("Config account has an invalid key");
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(())
    }
    // ----------------------- INITIALIZE ------------------------  
    pub unsafe fn initialize(
        account_info: &AccountInfo,
        bump: u8,
        base: &Pubkey,
        admin: &Pubkey,
        server: &Pubkey,
        game_fee_bps: u32,
    ) -> Result<(), ProgramError> {
        let mut data = account_info.borrow_mut_data_unchecked();
        let account = load_account_mut_unchecked::<Config>(&mut data)?;
 
        if account.is_initialized() {
            log!("Config account is already initialized");
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        account.discriminator = GameEngineDiscriminator::Config as u8;
        account.bump = bump;
        account.base = *base;
        account.admin = *admin;
        account.server = *server;
        account.game_fee_bps = game_fee_bps;

        Ok(())
    }

    // ----------------------- GETTERS ---------------------------
    
    pub fn bump(&self) -> u8 {
        self.bump
    }

    pub fn base(&self) -> &Pubkey {
        &self.base
    }

    pub fn admin(&self) -> &Pubkey {
        &self.admin
    }

    pub fn server(&self) -> &Pubkey {
        &self.server
    }
    
    pub fn game_fee_bps(&self) -> u32 {
        self.game_fee_bps
    }

    // ----------------------- SETTERS ---------------------------

} 