use crate::{
    accounts::config::Config,
    utils::{load_ix_data, load_signer, DataLen},
};
use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use super::ExampleProgramInstructions;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UpdateConfigIxData {
    pub discriminator: u8,
    pub new_admin: Option<Pubkey>,
    pub new_fees_bps: Option<u64>,
}

impl UpdateConfigIxData {
    pub fn new(new_admin: Option<Pubkey>, new_fees_bps: Option<u64>) -> Self {
        Self {
            discriminator: ExampleProgramInstructions::UpdateConfig as u8,
            new_admin,
            new_fees_bps,
        }
    }

    pub unsafe fn to_bytes(&self) -> &[u8] {
        unsafe { crate::utils::to_bytes::<Self>(&self) }
    }
}

impl DataLen for UpdateConfigIxData {
    const LEN: usize = core::mem::size_of::<UpdateConfigIxData>();
}

pub fn process_update_config(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let [config, admin] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let ix_data = unsafe { load_ix_data::<UpdateConfigIxData>(data)? };

    // ----------------------- CHECKS -----------------------
    load_signer(admin, true)?;
    Config::load(program_id, &config, true, Some(admin))?;
    unsafe {
        Config::check_admin(config, admin)?;
    }

    // ----------------------- WORK -----------------------
    unsafe {
        Config::update(config, &ix_data)?;
    }

    Ok(())
}
