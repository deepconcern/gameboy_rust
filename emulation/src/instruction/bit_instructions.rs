use crate::{Emulator, instruction::{Instruction, OpResult}, opcode::Opcode, flag::Flag};

fn bit(value: u8, bit_index: usize) -> bool {
    let mask = 0x01u8 << bit_index;

    let bit = value & mask;

    bit > 0
}

/// BIT b, (HL)
/// 
/// Z <- (HL)b
/// 
/// Copies the complement of the contents of the specified bit in memory
/// specified by register pair HL to the Z flag.
pub fn bit_complement_of_hl_location(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let bit_index = opcode.parse_bit(0b00_111_000);

    let value = emulator.read_hl_location()?;

    let bit = bit(value, bit_index);

    emulator.set_flag(Flag::H, true);
    emulator.set_flag(Flag::N, false);
    emulator.set_flag(Flag::Z, !bit);

    Ok(())
}

pub const BIT_COMPLEMENT_OF_HL_LOCATION: Instruction = Instruction {
    name: "BIT b, (HL)",
    op: bit_complement_of_hl_location,
    pattern: "01 bbb 110",
    requires_prefix: true,
};

/// BIT b, r
/// 
/// Z <- rb
/// 
/// Copies the complement of the contents of the specified bit in register r to
/// the Z flag.
pub fn bit_complement_of_register(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register = opcode.parse_register(0b00_000_111)?;
    let bit_index = opcode.parse_bit(0b00_111_000);

    let bit = bit(emulator.register(&register), bit_index);

    emulator.set_flag(Flag::H, true);
    emulator.set_flag(Flag::N, false);
    emulator.set_flag(Flag::Z, !bit);

    Ok(())
}

pub const BIT_COMPLEMENT_OF_REGISTER: Instruction = Instruction {
    name: "BIT b, r",
    op: bit_complement_of_register,
    pattern: "01 bbb rrr",
    requires_prefix: true,
};

/// RES b, (HL)
/// 
/// (HL)b <- 1
/// 
/// Sets to 0 the specified bit in memory specified by register pair HL.
pub fn reset_bit_of_hl_location(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let bit_index = opcode.parse_bit(0b00_111_000);

    let bit = !(0x01u8 << bit_index);

    let value = emulator.read_hl_location()?;

    emulator.write_hl_location(value & bit)?;

    Ok(())
}

pub const RESET_BIT_OF_HL_LOCATION: Instruction = Instruction {
    name: "RES b, (HL)",
    op: reset_bit_of_hl_location,
    pattern: "10 bbb 110",
    requires_prefix: true,
};

/// RES b, r
/// 
/// rb <- 1
/// 
/// Sets to 0 the specified bit in register r.
pub fn reset_bit_of_register(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register = opcode.parse_register(0b_000_111)?;

    let bit_index = opcode.parse_bit(0b00_111_000);

    let bit = !(0x01u8 << bit_index);

    let value = emulator.register(&register);

    emulator.set_register(register, value & bit);

    Ok(())
}

pub const RESET_BIT_OF_REGISTER: Instruction = Instruction {
    name: "RES b, r",
    op: reset_bit_of_register,
    pattern: "10 bbb rrr",
    requires_prefix: true,
};

/// SET b, (HL)
/// 
/// (HL)b <- 1
/// 
/// Sets to 1 the specified bit in memory specified by register pair HL.
pub fn set_bit_of_hl_location(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let bit_index = opcode.parse_bit(0b00_111_000);

    let bit = 0x01u8 << bit_index;

    let value = emulator.read_hl_location()?;

    emulator.write_hl_location(value | bit)?;

    Ok(())
}

pub const SET_BIT_OF_HL_LOCATION: Instruction = Instruction {
    name: "SET b, (HL)",
    op: set_bit_of_hl_location,
    pattern: "11 bbb 110",
    requires_prefix: true,
};

/// SET b, r
/// 
/// rb <- 1
/// 
/// Sets to 1 the specified bit in register r.
pub fn set_bit_of_register(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register = opcode.parse_register(0b_000_111)?;

    let bit_index = opcode.parse_bit(0b00_111_000);

    let bit = 0x01u8 << bit_index;

    let value = emulator.register(&register);

    emulator.set_register(register, value | bit);

    Ok(())
}

pub const SET_BIT_OF_REGISTER: Instruction = Instruction {
    name: "SET b, r",
    op: set_bit_of_register,
    pattern: "11 bbb rrr",
    requires_prefix: true,
};

pub fn add_bit_instructions(emulator: &mut Emulator) {
    emulator.add_instruction(BIT_COMPLEMENT_OF_HL_LOCATION);
    emulator.add_instruction(BIT_COMPLEMENT_OF_REGISTER);
    emulator.add_instruction(RESET_BIT_OF_HL_LOCATION);
    emulator.add_instruction(RESET_BIT_OF_REGISTER);
    emulator.add_instruction(SET_BIT_OF_HL_LOCATION);
    emulator.add_instruction(SET_BIT_OF_REGISTER);
}