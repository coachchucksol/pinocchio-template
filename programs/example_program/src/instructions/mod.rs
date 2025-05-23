pub mod initialize_config;
pub mod update_config;

use crate::errors::ExampleProgramError;

#[repr(u8)]
pub enum ExampleProgramInstructions {
    InitializeConfig = 1,
    UpdateConfig = 2,
}

impl TryFrom<&u8> for ExampleProgramInstructions {
    type Error = ExampleProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            1 => Ok(ExampleProgramInstructions::InitializeConfig),
            2 => Ok(ExampleProgramInstructions::UpdateConfig),
            _ => Err(ExampleProgramError::InvalidInstruction),
        }
    }
}
