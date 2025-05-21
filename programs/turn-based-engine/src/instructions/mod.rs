pub mod initialize_config;

use crate::errors::GameEngineError;

#[repr(u8)]
pub enum GameEngineInstructions {
    InitializeConfig,
}

impl TryFrom<&u8> for GameEngineInstructions {
    type Error = GameEngineError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(GameEngineInstructions::InitializeConfig),
            _ => Err(GameEngineError::InvalidInstruction),
        }
    }
}
