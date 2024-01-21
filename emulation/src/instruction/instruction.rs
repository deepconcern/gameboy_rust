use std::fmt::Display;

use crate::{
    emulator::Emulator,
    memory_component::MemoryError,
    opcode::{OpcodeError, OpcodePattern},
};

#[derive(Clone, Debug)]
pub enum OpError {
    ConditionParse(u8),
    Memory(MemoryError),
    PageParse(u8),
    RegisterParse(u8),
    RegisterPairParse(u8),
    Unimplemented(bool, u8),
}

pub type OpResult = Result<(), OpError>;

impl Display for OpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpError::ConditionParse(c) => write!(f, "invalid condition argument: {}", c),
            OpError::Memory(e) => e.fmt(f),
            OpError::PageParse(p) => write!(f, "invalid page argument: {}", p),
            OpError::RegisterParse(r) => write!(f, "invalid register argument: {}", r),
            OpError::RegisterPairParse(r) => write!(f, "invalid register pair argument: {}", r),
            OpError::Unimplemented(p, o) => {
                let opcode_type = if *p {
                    "prefix required"
                } else {
                    "no prefix required"
                };

                write!(f, "unimplemented opcode {:#04x} ({})", o, opcode_type)
            },
        }
    }
}

impl From<MemoryError> for OpError {
    fn from(value: MemoryError) -> Self {
        OpError::Memory(value)
    }
}

impl From<OpcodeError> for OpError {
    fn from(value: OpcodeError) -> Self {
        match value {
            OpcodeError::ConditionParse(a) => OpError::ConditionParse(a),
            OpcodeError::PageParse(a) => OpError::PageParse(a),
            OpcodeError::RegisterPairParse(a) => OpError::RegisterPairParse(a),
            OpcodeError::RegisterParse(a) => OpError::RegisterParse(a),
        }
    }
}

pub type Op = fn(&mut Emulator, u8) -> OpResult;

pub struct Instruction {
    pub name: &'static str,
    pub op: Op,
    pub pattern: &'static str,
    pub requires_prefix: bool,
}

impl Instruction {
    pub fn opcodes(&self) -> Vec<u8> {
        self.pattern.opcodes()
    }
}