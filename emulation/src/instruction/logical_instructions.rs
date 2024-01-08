use crate::{
    emulator::Emulator,
    instruction::{Instruction, OpResult},
    opcode::Opcode,
    register::RegisterPair,
};

/// AND (HL)
///
/// A <- A ∧ (HL)
///
/// Takes the logical-AND of the contents of register A and the memory specified
/// by register pair HL, and stores the results in register A.
pub fn and_hl_location(emulator: &mut Emulator, _: u8) -> OpResult {
    let location = emulator.register_pair(&RegisterPair::Hl);

    let value = emulator.read(location)?;

    emulator.bitwise_and_with_a(value);

    Ok(())
}

pub const AND_HL_LOCATION: Instruction = Instruction {
    name: "AND (HL)",
    op: and_hl_location,
    pattern: "10 100 110",
    requires_prefix: false,
};

/// AND n
///
/// A <- A ∧ n
///
/// Takes the logical-AND of the contents of register A and 8-bit immediate
/// operand n, and stores the results in register A.
pub fn and_immediate_n(emulator: &mut Emulator, _: u8) -> OpResult {
    let n = emulator.read_immediate_n()?;

    emulator.bitwise_and_with_a(n);

    Ok(())
}

pub const AND_IMMEDIATE_N: Instruction = Instruction {
    name: "AND n",
    op: and_immediate_n,
    pattern: "11 100 110",
    requires_prefix: false,
};

/// AND r
///
/// A <- A ∧ r
///
/// Takes the logical-AND of the contents of register A and register r, and
/// stores the results in register A.
pub fn and_register(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register = opcode.parse_register(0b00_000_111)?;

    let value = emulator.register(&register);

    emulator.bitwise_and_with_a(value);

    Ok(())
}

pub const AND_REGISTER: Instruction = Instruction {
    name: "AND r",
    op: and_register,
    pattern: "10 100 rrr",
    requires_prefix: false,
};

/// OR (HL)
///
/// A <- A v (HL)
///
/// Takes the logical-OR of the contents of register A and the memory specified
/// by register pair HL, and stores the results in register A.
pub fn or_hl_location(emulator: &mut Emulator, _: u8) -> OpResult {
    let location = emulator.register_pair(&RegisterPair::Hl);

    let value = emulator.read(location)?;

    emulator.bitwise_or_with_a(value);

    Ok(())
}

pub const OR_HL_LOCATION: Instruction = Instruction {
    name: "OR (HL)",
    op: or_hl_location,
    pattern: "10 110 110",
    requires_prefix: false,
};

/// OR n
///
/// A <- A v n
///
/// Takes the logical-OR of the contents of register A and 8-bit immediate
/// operand n, and stores the results in register A.
pub fn or_immediate_n(emulator: &mut Emulator, _: u8) -> OpResult {
    let n = emulator.read_immediate_n()?;

    emulator.bitwise_or_with_a(n);

    Ok(())
}

pub const OR_IMMEDIATE_N: Instruction = Instruction {
    name: "OR n",
    op: or_immediate_n,
    pattern: "11 110 110",
    requires_prefix: false,
};

/// OR r
///
/// A <- A v r
///
/// Takes the logical-OR of the contents of register A and register r, and
/// stores the results in register A.
pub fn or_register(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register = opcode.parse_register(0b00_000_111)?;

    let value = emulator.register(&register);

    emulator.bitwise_or_with_a(value);

    Ok(())
}

pub const OR_REGISTER: Instruction = Instruction {
    name: "OR r",
    op: or_register,
    pattern: "10 110 rrr",
    requires_prefix: false,
};

/// XOR (HL)
///
/// A <- A ⊕ (HL)
///
/// Takes the logical-XOR of the contents of register A and the memory specified
/// by register pair HL, and stores the results in register A.
pub fn xor_hl_location(emulator: &mut Emulator, _: u8) -> OpResult {
    let location = emulator.register_pair(&RegisterPair::Hl);

    let value = emulator.read(location)?;

    emulator.bitwise_xor_with_a(value);

    Ok(())
}

pub const XOR_HL_LOCATION: Instruction = Instruction {
    name: "XOR (HL)",
    op: xor_hl_location,
    pattern: "10 101 110",
    requires_prefix: false,
};

/// XOR n
///
/// A <- A ⊕ n
///
/// Takes the logical-XOR of the contents of register A and 8-bit immediate
/// operand n, and stores the results in register A.
pub fn xor_immediate_n(emulator: &mut Emulator, _: u8) -> OpResult {
    let n = emulator.read_immediate_n()?;

    emulator.bitwise_xor_with_a(n);

    Ok(())
}

pub const XOR_IMMEDIATE_N: Instruction = Instruction {
    name: "XOR n",
    op: xor_immediate_n,
    pattern: "11 101 110",
    requires_prefix: false,
};

/// XOR r
///
/// A <- A ⊕ r
///
/// Takes the logical-XOR of the contents of register A and register r, and
/// stores the results in register A.
pub fn xor_register(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register = opcode.parse_register(0b00_000_111)?;

    let value = emulator.register(&register);

    emulator.bitwise_xor_with_a(value);

    Ok(())
}

pub const XOR_REGISTER: Instruction = Instruction {
    name: "XOR r",
    op: xor_register,
    pattern: "10 101 rrr",
    requires_prefix: false,
};

pub fn add_logical_instructions(emulator: &mut Emulator) {
    emulator.add_instruction(AND_HL_LOCATION);
    emulator.add_instruction(AND_IMMEDIATE_N);
    emulator.add_instruction(AND_REGISTER);
    emulator.add_instruction(OR_HL_LOCATION);
    emulator.add_instruction(OR_IMMEDIATE_N);
    emulator.add_instruction(OR_REGISTER);
    emulator.add_instruction(XOR_HL_LOCATION);
    emulator.add_instruction(XOR_IMMEDIATE_N);
    emulator.add_instruction(XOR_REGISTER);
}
