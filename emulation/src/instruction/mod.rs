pub mod arithmetic_instructions;
pub mod bit_instructions;
pub mod call_instructions;
pub mod general_instructions;
mod instruction;
pub mod jump_instructions;
pub mod loading_instructions;
pub mod logical_instructions;
pub mod rotating_instructions;

pub use instruction::{Instruction, Op, OpError, OpResult};
