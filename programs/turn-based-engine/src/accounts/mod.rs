use pinocchio::program_error::ProgramError;
use pinocchio_log::log;

pub mod config;


#[repr(u8)]
pub enum GameEngineDiscriminator {
    Config = 1,
}

impl GameEngineDiscriminator {
    pub fn from_u8(value: u8) -> Result<Self, ProgramError> {
        match value {
            1 => Ok(GameEngineDiscriminator::Config),
            _ => {
                log!("Invalid account discriminator: {}", value);
                Err(ProgramError::InvalidInstructionData)
            }
        }
    }
}