use std::fmt::Display;

use num::FromPrimitive;

use crate::memory_component::MemoryError;
use crate::processor_state::ProcessorState;
use crate::register::{Register, RegisterPair};

#[derive(Clone, Debug)]
pub enum InstructionError {
    MemoryError(MemoryError),
    RegisterError(u8),
    RegisterPairError(u8),
}

impl Display for InstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionError::MemoryError(e) => e.fmt(f),
            InstructionError::RegisterError(r) => write!(f, "invalid register argument: {}", r),
            InstructionError::RegisterPairError(r) => write!(f, "invalid register pair argument: {}", r),
        }
    }
}

impl From<MemoryError> for InstructionError {
    fn from(value: MemoryError) -> Self {
        InstructionError::MemoryError(value)
    }
}

pub trait Instruction {
    fn cycles(&self) -> usize;

    fn name(&self) -> String;

    fn operation(&self, processor_state: &mut ProcessorState, opcode: u8) -> Result<(), InstructionError>;

    fn variations(&self) -> Vec<u8>;
}

pub fn parse_register_argument(opcode: &u8, mask: u8) -> Result<Register, InstructionError> {
    let argument = opcode & mask;

    Register::from_u8(argument).ok_or(InstructionError::RegisterError(argument))
}

pub fn parse_register_pair_argument(opcode: &u8, mask: &u8) -> Result<RegisterPair, InstructionError> {
    let argument = opcode & mask;

    RegisterPair::from_u8(argument).ok_or(InstructionError::RegisterPairError(argument))
}