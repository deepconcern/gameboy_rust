use crate::{
    emulator::Emulator,
    instruction::{Instruction, OpResult}, opcode::Opcode,
};

/// JP (HL)
/// 
/// PC <- HL
/// 
/// Loads the contents of register pair HL into pc.
pub fn jump_to_hl(emulator: &mut Emulator, _:u8) -> OpResult {
    emulator.jump_to(emulator.hl());

    Ok(())
}

pub const JUMP_TO_HL: Instruction = Instruction {
    name: "JP (HL)",
    op: jump_to_hl,
    pattern: "11 101 001",
    requires_prefix: false,
};

/// JP e
/// 
/// PC <- PC + e
/// 
/// Adds 8-bit immediate operand e with pc and loads the result into pc.
pub fn jump_to_immediate_e(emulator: &mut Emulator, _: u8) -> OpResult {
    let e = emulator.read_immediate_e()?;

    emulator.jump_relative_to(e);

    Ok(())
}

pub const JUMP_TO_IMMEDIATE_E: Instruction = Instruction {
    name: "JP e",
    op: jump_to_immediate_e,
    pattern: "00 011 000",
    requires_prefix: false,
};

/// JP cc, e
/// 
/// if cc, PC <- PC + e
/// 
/// Adds 8-bit immediate operand e with pc and loads the result into pc if cc is
/// true.
pub fn jump_to_immediate_e_if_condition(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let condition = opcode.parse_condition(0b00_011_000)?;

    let e = emulator.read_immediate_e()?;

    if condition.check(emulator) {
        emulator.jump_relative_to(e);
    };

    Ok(())
}

pub const JUMP_TO_IMMEDIATE_E_IF_CONDITION: Instruction = Instruction {
    name: "JP cc, e",
    op: jump_to_immediate_e_if_condition,
    pattern: "00 1cc 000",
    requires_prefix: false,
};

/// JP nn
/// 
/// PC <- nn
/// 
/// Loads 16-bit operand nn into pc.
pub fn jump_to_immediate_nn(emulator: &mut Emulator, _: u8) -> OpResult {
    let nn = emulator.read_immediate_nn()?;

    emulator.jump_to(nn);

    Ok(())
}

pub const JUMP_TO_IMMEDIATE_NN: Instruction = Instruction {
    name: "JP nn",
    op: jump_to_immediate_nn,
    pattern: "11 000 011",
    requires_prefix: false,
};

/// JP cc, nn
/// 
/// if cc, PC <- nn
/// 
/// Loads 16-bit operand nn into pc if cc is true.
pub fn jump_to_immediate_nn_if_condition(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let condition = opcode.parse_condition(0b00_011_000)?;

    let nn = emulator.read_immediate_nn()?;

    if condition.check(emulator) {
        emulator.jump_to(nn);
    };

    Ok(())
}

pub const JUMP_TO_IMMEDIATE_NN_IF_CONDITION: Instruction = Instruction {
    name: "JP cc, nn",
    op: jump_to_immediate_nn_if_condition,
    pattern: "11 0cc 010",
    requires_prefix: false,
};

pub fn add_jump_instructions(emulator: &mut Emulator) {
    emulator.add_instruction(JUMP_TO_HL);
    emulator.add_instruction(JUMP_TO_IMMEDIATE_E);
    emulator.add_instruction(JUMP_TO_IMMEDIATE_E_IF_CONDITION);
    emulator.add_instruction(JUMP_TO_IMMEDIATE_NN);
    emulator.add_instruction(JUMP_TO_IMMEDIATE_NN_IF_CONDITION);
}