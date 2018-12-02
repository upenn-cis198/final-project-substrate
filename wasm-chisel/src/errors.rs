use std::error;
use std::fmt;

#[derive(Debug)]
pub enum InstructionError {
	GlobalNotFound,
	LocalNotFound,
	UnmatchedInstruction,
	InvalidBinaryOperation,
}

impl fmt::Display for InstructionError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			InstructionError::GlobalNotFound =>
				write!(f, "global not found"),
			InstructionError::LocalNotFound =>
				write!(f, "local not found"),
			InstructionError::UnmatchedInstruction =>
				write!(f, "unmatched instruction"),
			InstructionError::InvalidBinaryOperation =>
				write!(f, "invalid binary operation"),
		}
	}
}

impl error::Error for InstructionError {
	fn description(&self) -> &str {
		match *self {
			InstructionError::GlobalNotFound =>
				"global not found",
			InstructionError::LocalNotFound =>
				"local not found",
			InstructionError::UnmatchedInstruction =>
				"unmatched instruction",
			InstructionError::InvalidBinaryOperation =>
				"invalid binary operation",
		}
	}

	fn cause(&self) -> Option<&error::Error> {
		None
	}
}
