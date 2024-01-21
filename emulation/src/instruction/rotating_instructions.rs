use crate::{
    emulator::Emulator,
    flag::Flag,
    instruction::{Instruction, OpResult},
    opcode::Opcode,
};

fn set_rotation_flags(emulator: &mut Emulator, value: u8) {
    emulator.set_flag(Flag::H, false);
    emulator.set_flag(Flag::N, false);
    emulator.set_flag(Flag::Z, value == 0);
}

fn rotate_value_left(emulator: &mut Emulator, value: u8, with_copy: bool) -> u8 {
    let bit = if with_copy {
        let a7 = (value & 0x80u8) > 0;

        emulator.set_flag(Flag::CY, a7);

        a7 as u8
    } else {
        let cy = emulator.flag(Flag::CY);

        emulator.set_flag(Flag::CY, (value & 0x80u8) > 0);

        cy as u8
    };

    let rotated_value = (value << 1) + bit;

    set_rotation_flags(emulator, rotated_value);

    rotated_value
}

fn rotate_value_right(emulator: &mut Emulator, value: u8, with_copy: bool) -> u8 {
    let bit = if with_copy {
        let a0 = (value & 0x001u8) > 0;

        emulator.set_flag(Flag::CY, a0);

        (a0 as u8) << 7
    } else {
        let cy = emulator.flag(Flag::CY);

        emulator.set_flag(Flag::CY, (value & 0x001u8) > 0);

        (cy as u8) << 7
    };

    let rotated_value = (value >> 1) + bit;

    set_rotation_flags(emulator, rotated_value);

    rotated_value
}

fn shift_value_left(emulator: &mut Emulator, value: u8) -> u8 {
    let mut shifted_value = value;

    emulator.set_flag(Flag::CY, shifted_value & 0x80 > 0);

    shifted_value = shifted_value << 1;

    set_rotation_flags(emulator, shifted_value);

    shifted_value
}

fn shift_value_right(emulator: &mut Emulator, value: u8, copy_bit: bool) -> u8 {
    let mut shifted_value = value;

    let v7 = shifted_value & 0x80 > 0;

    emulator.set_flag(Flag::CY, v7);

    shifted_value = shifted_value >> 1;

    if copy_bit {
        shifted_value = shifted_value + ((v7 as u8) << 7);
    };

    set_rotation_flags(emulator, shifted_value);

    shifted_value
}

/// RLCA
/// 
/// CY <- A7
/// 
/// A0 <- 7-A-0 <- A7
/// 
/// Rotates the contents of register A to the left. Bit 7 of the contents is
/// copied to both the CY flag and the bit 0 of the contents.
pub fn rotate_a_left_copy_carry(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.a();

    let rotated_value = rotate_value_left(emulator, value, true);

    emulator.set_a(rotated_value);

    emulator.set_flag(Flag::Z, false);

    Ok(())
}

pub const ROTATE_A_LEFT_COPY_CARRY: Instruction = Instruction {
    name: "RLCA",
    op: rotate_a_left_copy_carry,
    pattern: "00 000 111",
    requires_prefix: false,
};

/// RLA
/// 
/// A0 <- CY <- 7-A-0
/// 
/// Rotates the contents of register A to the left. Bit 7 of the contents is
/// stored in the CY flag, and the contents of the CY flag are stored in bit 0
/// of the contents.
pub fn rotate_a_left_through_carry(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.a();

    let rotated_value = rotate_value_left(emulator, value, false);

    emulator.set_a(rotated_value);

    emulator.set_flag(Flag::Z, false);

    Ok(())
}

pub const ROTATE_A_LEFT_THROUGH_CARRY: Instruction = Instruction {
    name: "RLA",
    op: rotate_a_left_through_carry,
    pattern: "00 010 111",
    requires_prefix: false,
};

/// RRCA
/// 
/// CY <- A0
/// 
/// A7 <- 0-A-7 <- A0
/// 
/// Rotates the contents of register A to the right. Bit 0 of the contents is
/// copied to both the CY flag and the bit 7 of the contents.
pub fn rotate_a_right_copy_carry(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.a();

    let rotated_value = rotate_value_right(emulator, value, true);

    emulator.set_a(rotated_value);

    emulator.set_flag(Flag::Z, false);

    Ok(())
}

pub const ROTATE_A_RIGHT_COPY_CARRY: Instruction = Instruction {
    name: "RRCA",
    op: rotate_a_right_copy_carry,
    pattern: "00 001 111",
    requires_prefix: false,
};

/// RRA
/// 
/// A7 <- CY <- 0-A-7
/// 
/// Rotates the contents of register A to the right. Bit 0 of the contents is
/// stored in the CY flag, and the contents of the CY flag are stored in bit 7
/// of the contents.
pub fn rotate_a_right_through_carry(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.a();

    let rotated_value = rotate_value_right(emulator, value, false);

    emulator.set_a(rotated_value);

    emulator.set_flag(Flag::Z, false);

    Ok(())
}

pub const ROTATE_A_RIGHT_THROUGH_CARRY: Instruction = Instruction {
    name: "RRA",
    op: rotate_a_right_through_carry,
    pattern: "00 011 111",
    requires_prefix: false,
};

/// RLC (HL)
/// 
/// CY <- (HL)7
/// 
/// (HL)0 <- 7-(HL)-0 <- (HL)7
/// 
/// Rotates the contents of memory specified by the register pair HL to the
/// left. Bit 7 of the contents is copied to both the CY flag and the bit 0 of
/// contents.
pub fn rotate_hl_location_left_copy_carry(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.read_hl_location()?;

    let rotated_value = rotate_value_left(emulator, value, true);

    emulator.write_hl_location(rotated_value)?;

    Ok(())
}

pub const ROTATE_HL_LOCATION_LEFT_COPY_CARRY: Instruction = Instruction {
    name: "RLC (HL)",
    op: rotate_hl_location_left_copy_carry,
    pattern: "00 000 110",
    requires_prefix: true,
};

/// RL (HL)
/// 
/// (HL)0 <- CY <- 7-(HL)-0
/// 
/// Rotates the contents of memory specified by the register pair HL to the
/// left. Bit 7 of the contents is stored in the CY flag, and the contents of
/// the CY flag are stored in bit 0 of the contents.
pub fn rotate_hl_location_left_through_carry(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.read_hl_location()?;

    let rotated_value = rotate_value_left(emulator, value, false);

    emulator.write_hl_location(rotated_value)?;

    Ok(())
}

pub const ROTATE_HL_LOCATION_LEFT_THROUGH_CARRY: Instruction = Instruction {
    name: "RL (HL)",
    op: rotate_hl_location_left_through_carry,
    pattern: "00 010 110",
    requires_prefix: true,
};

/// RRC (HL)
/// 
/// CY <- (HL)7
/// 
/// (HL)0 <- 7-(HL)-0 <- (HL)7
/// 
/// Rotates the contents of memory specified by the register pair HL to the
/// right. Bit 7 of the contents is copied to both the CY flag and the bit 0 of
/// contents.
pub fn rotate_hl_location_right_copy_carry(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.read_hl_location()?;

    let rotated_value = rotate_value_right(emulator, value, true);

    emulator.write_hl_location(rotated_value)?;

    Ok(())
}

pub const ROTATE_HL_LOCATION_RIGHT_COPY_CARRY: Instruction = Instruction {
    name: "RRC (HL)",
    op: rotate_hl_location_right_copy_carry,
    pattern: "00 001 110",
    requires_prefix: true,
};

/// RR (HL)
/// 
/// (HL)0 <- CY <- 7-(HL)-0
/// 
/// Rotates the contents of memory specified by the register pair HL to the
/// right. Bit 7 of the contents is stored in the CY flag, and the contents of
/// the CY flag are stored in bit 0 of the contents.
pub fn rotate_hl_location_right_through_carry(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.read_hl_location()?;

    let rotated_value = rotate_value_right(emulator, value, false);

    emulator.write_hl_location(rotated_value)?;

    Ok(())
}

pub const ROTATE_HL_LOCATION_RIGHT_THROUGH_CARRY: Instruction = Instruction {
    name: "RR (HL)",
    op: rotate_hl_location_right_through_carry,
    pattern: "00 011 110",
    requires_prefix: true,
};

/// RLC r
/// 
/// CY <- r7
/// 
/// r0 <- 7-r-0 <- r7
/// 
/// Rotates the contents of register r to the left. Bit 7 of the contents is
/// copied to both the CY flag and the bit 0 of contents.
pub fn rotate_register_left_copy_carry(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register = opcode.parse_register(0b00_000_111)?;

    let value = emulator.register(&register);

    let rotated_value = rotate_value_left(emulator, value, true);

    emulator.set_register(register, rotated_value);

    Ok(())
}

pub const ROTATE_REGISTER_LEFT_COPY_CARRY: Instruction = Instruction {
    name: "RLC r",
    op: rotate_register_left_copy_carry,
    pattern: "00 000 rrr",
    requires_prefix: true,
};

/// RL r
/// 
/// r0 <- CY <- 7-r-0
/// 
/// Rotates the contents of register r to the left. Bit 7 of the contents is
/// stored in the CY flag, and the contents of the CY flag are stored in bit 0
/// of the contents.
pub fn rotate_register_left_through_carry(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register = opcode.parse_register(0b00_000_111)?;

    let value = emulator.register(&register);

    let rotated_value = rotate_value_left(emulator, value, false);

    emulator.set_register(register, rotated_value);

    Ok(())
}

pub const ROTATE_REGISTER_LEFT_THROUGH_CARRY: Instruction = Instruction {
    name: "RL r",
    op: rotate_register_left_through_carry,
    pattern: "00 010 rrr",
    requires_prefix: true,
};

/// RRC r
/// 
/// CY <- r0
/// 
/// r7 <- 0-r-7 <- r0
/// 
/// Rotates the contents of register r to the right. Bit 0 of the contents is
/// copied to both the CY flag and the bit 7 of contents.
pub fn rotate_register_right_copy_carry(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register = opcode.parse_register(0b00_000_111)?;

    let value = emulator.register(&register);

    let rotated_value = rotate_value_right(emulator, value, true);

    emulator.set_register(register, rotated_value);

    Ok(())
}

pub const ROTATE_REGISTER_RIGHT_COPY_CARRY: Instruction = Instruction {
    name: "RRC r",
    op: rotate_register_right_copy_carry,
    pattern: "00 001 rrr",
    requires_prefix: true,
};

/// RR r
/// 
/// r7 <- CY <- 0-r-7
/// 
/// Rotates the contents of register r to the right. Bit 0 of the contents is
/// stored in the CY flag, and the contents of the CY flag are stored in bit 7
/// of the contents.
pub fn rotate_register_right_through_carry(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register = opcode.parse_register(0b00_000_111)?;

    let value = emulator.register(&register);

    let rotated_value = rotate_value_right(emulator, value, false);

    emulator.set_register(register, rotated_value);

    Ok(())
}

pub const ROTATE_REGISTER_RIGHT_THROUGH_CARRY: Instruction = Instruction {
    name: "RR r",
    op: rotate_register_right_through_carry,
    pattern: "00 011 rrr",
    requires_prefix: true,
};

/// SLA (HL)
/// 
/// CY <- (HL)7
/// 
/// (HL) <- 6-(HL)-0, 0
/// 
/// Shift the contents of memory specified by the register pair HL to the left. 
/// Bit 7 is stored in CY and zero is inserted at bit 0.
pub fn shift_hl_location_left(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.read_hl_location()?;

    let shifted_value = shift_value_left(emulator, value);

    emulator.write_hl_location(shifted_value)?;

    Ok(())
}

pub const SHIFT_HL_LOCATION_LEFT: Instruction = Instruction {
    name: "SLA (HL)",
    op: shift_hl_location_left,
    pattern: "00 100 110",
    requires_prefix: true,
};

/// SRA (HL)
/// 
/// CY <- (HL)0
/// 
/// (HL) <- 0, 7-(HL)-1
/// 
/// Shift the contents of memory specified by the register pair HL to the right.
/// Bit 0 is stored in CY and zero is inserted at bit 7.
pub fn shift_hl_location_right(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.read_hl_location()?;

    let shifted_value = shift_value_right(emulator, value, false);

    emulator.write_hl_location(shifted_value)?;

    Ok(())
}

pub const SHIFT_HL_LOCATION_RIGHT: Instruction = Instruction {
    name: "SRA (HL)",
    op: shift_hl_location_right,
    pattern: "00 101 110",
    requires_prefix: true,
};

/// SRL (HL)
/// 
/// CY <- (HL)0
/// 
/// (HL) <- (HL)7, 7-(HL)-1
/// 
/// Shift the contents of memory specified by the register pair HL to the right.
/// Bit 0 is stored in CY and r7 is inserted at bit 7.
pub fn shift_hl_location_right_copy_bit(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.read_hl_location()?;

    let shifted_value = shift_value_right(emulator, value, true);

    emulator.write_hl_location(shifted_value)?;

    Ok(())
}

pub const SHIFT_HL_LOCATION_RIGHT_COPY_BIT: Instruction = Instruction {
    name: "SRL (HL)",
    op: shift_hl_location_right_copy_bit,
    pattern: "00 111 110",
    requires_prefix: true,
};

/// SLA r
/// 
/// CY <- r7
/// 
/// r <- 6-r-0, 0
/// 
/// Shift the contents of register r to the left. Bit 7 is stored in CY and zero
/// is inserted at bit 0.
pub fn shift_register_left(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register = opcode.parse_register(0b00_000_111)?;

    let value = emulator.register(&register);

    let shifted_value = shift_value_left(emulator, value);

    emulator.set_register(register, shifted_value);

    Ok(())
}

pub const SHIFT_REGISTER_LEFT: Instruction = Instruction {
    name: "SLA r",
    op: shift_register_left,
    pattern: "00 100 rrr",
    requires_prefix: true,
};

/// SRA r
/// 
/// CY <- r0
/// 
/// r <- 0, 7-r-1
/// 
/// Shift the contents of register r to the right. Bit 0 is stored in CY and
/// zero is inserted at bit 7.
pub fn shift_register_right(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register = opcode.parse_register(0b00_000_111)?;

    let value = emulator.register(&register);

    let shifted_value = shift_value_right(emulator, value, false);

    emulator.set_register(register, shifted_value);

    Ok(())
}

pub const SHIFT_REGISTER_RIGHT: Instruction = Instruction {
    name: "SRA r",
    op: shift_register_right,
    pattern: "00 101 rrr",
    requires_prefix: true,
};

/// SRA r
/// 
/// CY <- r0
/// 
/// r <- r7, 7-r-1
/// 
/// Shift the contents of register r to the right. Bit 0 is stored in CY and r7
/// is inserted at bit 7.
pub fn shift_register_right_copy_bit(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register = opcode.parse_register(0b00_000_111)?;

    let value = emulator.register(&register);

    let shifted_value = shift_value_right(emulator, value, true);

    emulator.set_register(register, shifted_value);

    Ok(())
}

pub const SHIFT_REGISTER_RIGHT_COPY_BIT: Instruction = Instruction {
    name: "SRL r",
    op: shift_register_right_copy_bit,
    pattern: "00 111 rrr",
    requires_prefix: true,
};

/// SWAP (HL)
/// 
/// (HL) <- (HL)H <-> (HL)L
/// 
/// Swaps the high bit and low bit of memory specified by register pair HL.
pub fn swap_bits_of_hl_location(emulator: &mut Emulator, _: u8) -> OpResult {
    let mut value = emulator.read_hl_location()?;

    let low_value = value & 0x0fu8;

    value = (value >> 4) + (low_value << 4);

    emulator.write_hl_location(value)?;

    emulator.set_flag(Flag::CY, false);
    emulator.set_flag(Flag::H, false);
    emulator.set_flag(Flag::N, false);
    emulator.set_flag(Flag::Z, value == 0);

    Ok(())
}

pub const SWAP_BITS_OF_HL_LOCATION: Instruction = Instruction {
    name: "SWAP (HL)",
    op: swap_bits_of_hl_location,
    pattern: "00 110 110",
    requires_prefix: true,
};

/// SWAP r
/// 
/// r <- rH <-> rL
/// 
/// Swaps the high bit and low bit of register r.
pub fn swap_bits_of_register(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register = opcode.parse_register(0b00_000_111)?;

    let mut value = emulator.register(&register);

    let low_value = value & 0x0fu8;

    value = (value >> 4) + (low_value << 4);

    emulator.set_register(register, value);

    emulator.set_flag(Flag::CY, false);
    emulator.set_flag(Flag::H, false);
    emulator.set_flag(Flag::N, false);
    emulator.set_flag(Flag::Z, value == 0);

    Ok(())
}

pub const SWAP_BITS_OF_REGISTER: Instruction = Instruction {
    name: "SWAP r",
    op: swap_bits_of_register,
    pattern: "00 110 rrr",
    requires_prefix: true,
};

pub fn add_rotating_instructions(emulator: &mut Emulator) {
    emulator.add_instruction(ROTATE_A_LEFT_COPY_CARRY);
    emulator.add_instruction(ROTATE_A_LEFT_THROUGH_CARRY);
    emulator.add_instruction(ROTATE_A_RIGHT_COPY_CARRY);
    emulator.add_instruction(ROTATE_A_RIGHT_THROUGH_CARRY);
    emulator.add_instruction(ROTATE_HL_LOCATION_LEFT_COPY_CARRY);
    emulator.add_instruction(ROTATE_HL_LOCATION_LEFT_THROUGH_CARRY);
    emulator.add_instruction(ROTATE_HL_LOCATION_RIGHT_COPY_CARRY);
    emulator.add_instruction(ROTATE_HL_LOCATION_RIGHT_THROUGH_CARRY);
    emulator.add_instruction(ROTATE_REGISTER_LEFT_COPY_CARRY);
    emulator.add_instruction(ROTATE_REGISTER_LEFT_THROUGH_CARRY);
    emulator.add_instruction(ROTATE_REGISTER_RIGHT_COPY_CARRY);
    emulator.add_instruction(ROTATE_REGISTER_RIGHT_THROUGH_CARRY);
    emulator.add_instruction(SHIFT_HL_LOCATION_LEFT);
    emulator.add_instruction(SHIFT_HL_LOCATION_RIGHT);
    emulator.add_instruction(SHIFT_HL_LOCATION_RIGHT_COPY_BIT);
    emulator.add_instruction(SHIFT_REGISTER_LEFT);
    emulator.add_instruction(SHIFT_REGISTER_RIGHT);
    emulator.add_instruction(SHIFT_REGISTER_RIGHT_COPY_BIT);
    emulator.add_instruction(SWAP_BITS_OF_HL_LOCATION);
    emulator.add_instruction(SWAP_BITS_OF_REGISTER);
}