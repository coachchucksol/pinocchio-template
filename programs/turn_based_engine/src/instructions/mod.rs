pub mod initialize_config;
pub mod update_config;

use crate::errors::GameEngineError;

#[repr(u8)]
pub enum GameEngineInstructions {
    InitializeConfig = 1,
    UpdateConfig = 2,
}

impl TryFrom<&u8> for GameEngineInstructions {
    type Error = GameEngineError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            1 => Ok(GameEngineInstructions::InitializeConfig),
            2 => Ok(GameEngineInstructions::UpdateConfig),
            _ => Err(GameEngineError::InvalidInstruction),
        }
    }
}
