use pinocchio::program_error::ProgramError;

#[derive(Clone, PartialEq)]
pub enum GameEngineError {
    InvalidInstruction,
    InvalidInstructionData,
    ArithmeticOverflow,
    ArithmeticUnderflow,
}

impl From<GameEngineError> for ProgramError {
    fn from(e: GameEngineError) -> Self {
        Self::Custom(e as u32)
    }
}