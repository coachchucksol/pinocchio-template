
use pinocchio::{account_info::AccountInfo, instruction::{Seed}, program_error::ProgramError, pubkey::{self, Pubkey}};
use pinocchio_log::log;

use crate::{accounts::GameEngineDiscriminator, instructions::{initialize_config::InitializeConfigIxData, update_config::UpdateConfigIxData}, utils::{load_account, load_account_mut, load_account_mut_unchecked, load_signer, DataLen, Initialized}};

/// The Counter account structure
#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
pub struct Config {
    discriminator: u8,
    bump: u8,
    base: Pubkey,
    admin: Pubkey,
    fees_bps: u64,
}

impl DataLen for Config {
    const LEN: usize = core::mem::size_of::<Config>();
}

impl Initialized for Config {
    fn is_initialized(&self) -> bool {
        self.discriminator == GameEngineDiscriminator::Config as u8
    }
}


#[macro_export]
macro_rules! config_seed_with_bump {
    ($base:expr, $bump_slice:expr) => {
        [crate::accounts::config::Config::SEED, $base.as_ref(), $bump_slice]
    };
}

impl Config {
    // ----------------------- ACCOUNT CHECKS ---------------------------
    pub const SEED: &[u8] = b"CONFIG";

    pub fn create_program_address(
        program_id: &Pubkey,
        base: &Pubkey,
        bump: u8,
    ) -> Result<Pubkey, ProgramError> {
        let bump_bytes = [bump];
        let seed_with_bump = config_seed_with_bump!(base, &bump_bytes);
        let pda = pubkey::create_program_address(&seed_with_bump, program_id)?;

        Ok(pda)
    }

    // Sanity check for the seeds
    pub fn check_seeds(
        base: &Pubkey,
        bump: u8,
        seeds: &[Seed],
    ) -> Result<(), ProgramError> {
        let bump_bytes = [bump];
        let seed_with_bump = config_seed_with_bump!(base, &bump_bytes);

        if seeds.len() != seed_with_bump.len() {
            return Err(ProgramError::InvalidAccountData);
        }

        for (seed_index, seed) in seeds.iter().enumerate(){
            for (byte_index, byte) in seed.as_ref().iter().enumerate() {
                let seed_byte = seed_with_bump[seed_index][byte_index];
                if byte.ne(&seed_byte) {
                    return Err(ProgramError::InvalidAccountData);
                }
            }
        }

        Ok(())
    }

    pub fn load(
        program_id: &Pubkey,
        account_info: &AccountInfo,
        expect_writable: bool,
        check_admin: Option<&AccountInfo>,
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

        if let Some(admin) = check_admin {
            load_signer(admin, true)?;
            if account.admin.ne(admin.key()) {
                log!("Config account has an invalid admin");
                return Err(ProgramError::InvalidAccountData);
            }
        }

        Ok(())
    }

    pub unsafe fn check_admin(
        account_info: &AccountInfo,
        admin: &AccountInfo,
    ) -> Result<(), ProgramError> {
        let mut data = account_info.borrow_mut_data_unchecked();
        let account = load_account_mut_unchecked::<Config>(&mut data)?;

        if account.admin.ne(admin.key()) {
            log!("Config account has an invalid admin");
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(())
    }
    // ----------------------- INITIALIZE ------------------------  
    pub unsafe fn initialize(
        account_info: &AccountInfo,
        base: &Pubkey,
        admin: &Pubkey,
        ix_data: &InitializeConfigIxData,
    ) -> Result<(), ProgramError> {
        let mut data = account_info.borrow_mut_data_unchecked();
        let account = load_account_mut_unchecked::<Config>(&mut data)?;
 
        if account.is_initialized() {
            log!("Config account is already initialized");
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        account.discriminator = GameEngineDiscriminator::Config as u8;
        account.bump = ix_data.config_bump;
        account.base = *base;
        account.admin = *admin;
        account.fees_bps = ix_data.fees_bps;

        Ok(())
    }

    // ----------------------- UPDATE ----------------------------
    pub unsafe fn update(
        account_info: &AccountInfo,
        ix_data: &UpdateConfigIxData,
    ) -> Result<(), ProgramError> {
        let mut data = account_info.borrow_mut_data_unchecked();
        let account = load_account_mut::<Config>(&mut data)?;
 
        if let Some(new_admin) = ix_data.new_admin {
            account.admin = new_admin;
        }


        if let Some(new_fees_bps) = ix_data.new_fees_bps {
            account.fees_bps = new_fees_bps;
        }

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

    pub fn fees_bps(&self) -> u64 {
        self.fees_bps
    }

    // ----------------------- SETTERS ---------------------------

} 