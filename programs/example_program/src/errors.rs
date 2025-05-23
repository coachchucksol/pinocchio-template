use pinocchio::program_error::ProgramError;

#[derive(Clone, PartialEq)]
pub enum ExampleProgramError {
    InvalidInstruction,
    InvalidInstructionData,
    ArithmeticOverflow,
    ArithmeticUnderflow,
}

impl From<ExampleProgramError> for ProgramError {
    fn from(e: ExampleProgramError) -> Self {
        Self::Custom(e as u32)
    }
}
