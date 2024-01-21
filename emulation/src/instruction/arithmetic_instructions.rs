use crate::{
    emulator::Emulator,
    instruction::{Instruction, OpResult},
    opcode::Opcode,
};

/// ADD A, (HL)
///
/// A <- A + (HL)
///
/// Adds the contents of memory specified by the contents of register pair HL
/// to the contents of register A and stores the results in register A.
pub fn add_hl_location_to_a(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.read_hl_location()?;

    emulator.add_to_a(value, false);

    Ok(())
}

pub const ADD_HL_LOCATION_TO_A: Instruction = Instruction {
    name: "ADD A, (HL)",
    op: add_hl_location_to_a,
    pattern: "10 000 110",
    requires_prefix: false,
};

/// ADD SP, e
/// 
/// SP <- SP + e
/// 
/// Adds 8-bit immediate operand e to the contents of the stack pointer and
/// stores the results in the stack pointer.
pub fn add_immediate_e_to_sp(emulator: &mut Emulator, _: u8) -> OpResult {
    let e = emulator.read_immediate_e()?;

    let value = emulator.add_signed(emulator.stack_pointer(), e, false);

    emulator.set_stack_pointer(value);

    Ok(())
}

pub const ADD_IMMEDIATE_E_TO_SP: Instruction = Instruction {
    name: "ADD SP, e",
    op: add_immediate_e_to_sp,
    pattern: "11 101 000",
    requires_prefix: false,
};

/// ADD A, n
///
/// A <- A + n
///
/// Adds 8-bit immediate operand n to the contents of register A and stores the
/// results in register A.
pub fn add_immediate_n_to_a(emulator: &mut Emulator, _: u8) -> OpResult {
    let n = emulator.read_immediate_n()?;

    emulator.add_to_a(n, false);

    Ok(())
}

pub const ADD_IMMEDIATE_N_TO_A: Instruction = Instruction {
    name: "ADD A, n",
    op: add_immediate_n_to_a,
    pattern: "11 000 110",
    requires_prefix: false,
};

/// ADD HL, ss
/// 
/// HL <- ss
/// 
/// Adds the contents of register pair ss to those of register pair HL and
/// stores the results in register pair HL.
pub fn add_register_pair_to_hl(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register_pair = opcode.parse_register_pair(0b00_110_000)?;

    let value = emulator.add_unsigned(emulator.hl(), emulator.register_pair(&register_pair), false);

    emulator.set_hl(value);

    Ok(())
}

pub const ADD_REGISTER_PAIR_TO_HL: Instruction = Instruction {
    name: "ADD HL, ss",
    op: add_register_pair_to_hl,
    pattern: "00 ss1 001",
    requires_prefix: false,
};

/// ADD A, r
///
/// A <- A + r
///
/// Adds the contents of register r to those of register A and stores the
/// results in register A.
pub fn add_register_to_a(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register = opcode.parse_register(0b00000111)?;

    let value = emulator.register(&register);

    emulator.add_to_a(value, false);

    Ok(())
}

pub const ADD_REGISTER_TO_A: Instruction = Instruction {
    name: "ADD A, r",
    op: add_register_to_a,
    pattern: "10 000 rrr",
    requires_prefix: false,
};

/// ADC A, (HL)
///
/// A <- A + (HL) + CY
///
/// Adds the contents of register pair HL and CY to the contents of register
/// A and stores the results in register A.
pub fn add_with_carry_hl_location_to_a(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.read_hl_location()?;

    emulator.add_to_a(value, true);

    Ok(())
}

pub const ADD_WITH_CARRY_HL_LOCATION_TO_A: Instruction = Instruction {
    name: "ADC A, (HL)",
    op: add_with_carry_hl_location_to_a,
    pattern: "10 001 110",
    requires_prefix: false,
};

/// ADC A, n
///
/// A <- A + n + CY
///
/// Adds 8-bit immediate operand n and CY to the contents of register A and
/// stores the results in register A.
pub fn add_with_carry_immediate_n_to_a(emulator: &mut Emulator, _: u8) -> OpResult {
    let n = emulator.read_immediate_n()?;

    emulator.add_to_a(n, true);

    Ok(())
}

pub const ADD_WITH_CARRY_IMMEDIATE_N_TO_A: Instruction = Instruction {
    name: "ADC A, n",
    op: add_with_carry_immediate_n_to_a,
    pattern: "11 001 110",
    requires_prefix: false,
};

/// ADC A, r
///
/// A <- A + r + CY
///
/// Adds the contents of register r and CY to those of register A and stores the
/// results in register A.
pub fn add_with_carry_register_to_a(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register = opcode.parse_register(0b00000111)?;

    let value = emulator.register(&register);

    emulator.add_to_a(value, true);

    Ok(())
}

pub const ADD_WITH_CARRY_REGISTER_TO_A: Instruction = Instruction {
    name: "ADC A, r",
    op: add_with_carry_register_to_a,
    pattern: "10 001 rrr",
    requires_prefix: false,
};

/// CP (HL)
/// 
/// A -- (HL)
/// 
/// Compares the contents of register a and memory specified by register pair
/// HL.
pub fn compare_hl_location_with_a(emulator: &mut Emulator, _: u8) -> OpResult {
    let a = emulator.a();
    let value = emulator.read_hl_location()?;

    emulator.subtract_unsigned(a, value, false);

    Ok(())
}

pub const COMPARE_HL_LOCATION_WITH_A: Instruction = Instruction {
    name: "CP (HL)",
    op: compare_hl_location_with_a,
    pattern: "10 111 110",
    requires_prefix: false,
};

/// CP n
/// 
/// A -- n
/// 
/// Compares the contents of register a and 8-bit immediate operand n.
pub fn compare_immediate_n_with_a(emulator: &mut Emulator, _: u8) -> OpResult {
    let a = emulator.a();
    let value = emulator.read_immediate_n()?;

    emulator.subtract_unsigned(a, value, false);

    Ok(())
}

pub const COMPARE_IMMEDIATE_N_WITH_A: Instruction = Instruction {
    name: "CP n",
    op: compare_immediate_n_with_a,
    pattern: "11 111 110",
    requires_prefix: false,
};

/// CP r
/// 
/// A -- r
/// 
/// Compares the contents of register a and register r.
pub fn compare_register_with_a(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register = opcode.parse_register(0b_00_000_111)?;

    let a = emulator.a();
    let value = emulator.register(&register);

    emulator.subtract_unsigned(a, value, false);

    Ok(())
}

pub const COMPARE_REGISTER_WITH_A: Instruction = Instruction {
    name: "CP r",
    op: compare_register_with_a,
    pattern: "10 111 rrr",
    requires_prefix: false,
};

/// DEC (HL)
/// 
/// (HL) <- (HL) - 1
/// 
/// Decrements the contents of memory specified by register pair HL.
pub fn dec_hl_location(emulator: &mut Emulator, _: u8) -> OpResult {
    let mut value = emulator.read_hl_location()?;

    value = emulator.subtract_unsigned(value, 1u8, false);

    emulator.write_hl_location(value)?;

    Ok(())
}

pub const DEC_HL_LOCATION: Instruction = Instruction {
    name: "DEC (HL)",
    op: dec_hl_location,
    pattern: "00 110 101",
    requires_prefix: false,
};

/// DEC r
/// 
/// r <- r - 1
/// 
/// Decrements the contents of register r.
pub fn dec_register(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register = opcode.parse_register(0b00_111_000u8)?;

    let mut value = emulator.register(&register);

    value = emulator.subtract_unsigned(value, 1u8, false);

    emulator.set_register(register, value);

    Ok(())
}

pub const DEC_REGISTER: Instruction = Instruction {
    name: "DEC r",
    op: dec_register,
    pattern: "00 rrr 101",
    requires_prefix: false,
};

/// DEC ss
/// 
/// ss <- ss - 1
/// 
/// Decrements the contents of register pair ss by 1.
pub fn dec_register_pair(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register_pair = opcode.parse_register_pair(0b00_110_000u8)?;

    let value = emulator.register_pair(&register_pair).wrapping_sub(1u16);

    emulator.set_register_pair(register_pair, value);

    Ok(())
}

pub const DEC_REGISTER_PAIR: Instruction = Instruction {
    name: "DEC ss",
    op: dec_register_pair,
    pattern: "00 ss1 011",
    requires_prefix: false,
};

/// INC (HL)
/// 
/// (HL) <- (HL) - 1
/// 
/// Increments the contents of memory specified by register pair HL.
pub fn inc_hl_location(emulator: &mut Emulator, _: u8) -> OpResult {
    let mut value = emulator.read_hl_location()?;

    value = emulator.add_unsigned(value, 1u8, false);

    emulator.write_hl_location(value)?;

    Ok(())
}

pub const INC_HL_LOCATION: Instruction = Instruction {
    name: "INC (HL)",
    op: inc_hl_location,
    pattern: "00 110 100",
    requires_prefix: false,
};

/// INC r
/// 
/// r <- r - 1
/// 
/// Increments the contents of register r.
pub fn inc_register(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register = opcode.parse_register(0b00_111_000u8)?;

    let mut value = emulator.register(&register);

    value = emulator.add_unsigned(value, 1u8, false);

    emulator.set_register(register, value);

    Ok(())
}

pub const INC_REGISTER: Instruction = Instruction {
    name: "INC r",
    op: inc_register,
    pattern: "00 rrr 100",
    requires_prefix: false,
};

/// INC ss
/// 
/// ss <- ss + 1
/// 
/// Increments the contents of register pair ss by 1.
pub fn inc_register_pair(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register_pair = opcode.parse_register_pair(0b00_110_000u8)?;

    let value = emulator.register_pair(&register_pair).wrapping_add(1u16);

    emulator.set_register_pair(register_pair, value);

    Ok(())
}

pub const INC_REGISTER_PAIR: Instruction = Instruction {
    name: "INC ss",
    op: inc_register_pair,
    pattern: "00 ss0 011",
    requires_prefix: false,
};

/// SUB A, (HL)
///
/// A <- A - (HL)
///
/// Subtracts the contents of memory specified by the contents of register pair
/// HL to the contents of register A and stores the results in register A.
pub fn sub_hl_location_from_a(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.read_hl_location()?;

    emulator.subtract_from_a(value, false);

    Ok(())
}

pub const SUB_HL_LOCATION_FROM_A: Instruction = Instruction {
    name: "SUB A, (HL)",
    op: sub_hl_location_from_a,
    pattern: "10 010 110",
    requires_prefix: false,
};

/// SUB A, n
///
/// A <- A - n
///
/// Subtracts 8-bit immediate operand n to the contents of register A and stores
/// the results in register A.
pub fn sub_immediate_n_from_a(emulator: &mut Emulator, _: u8) -> OpResult {
    let n = emulator.read_immediate_n()?;

    emulator.subtract_from_a(n, false);

    Ok(())
}

pub const SUB_IMMEDIATE_N_FROM_A: Instruction = Instruction {
    name: "SUB A, n",
    op: sub_immediate_n_from_a,
    pattern: "11 010 110",
    requires_prefix: false,
};

/// SUB A, r
///
/// A <- A - r
///
/// Subtracts the contents of register r to those of register A and stores the
/// results in register A.
pub fn sub_register_from_a(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register = opcode.parse_register(0b00000111)?;

    let value = emulator.register(&register);

    emulator.subtract_from_a(value, false);

    Ok(())
}

pub const SUB_REGISTER_FROM_A: Instruction = Instruction {
    name: "SUB A, r",
    op: sub_register_from_a,
    pattern: "10 010 rrr",
    requires_prefix: false,
};

/// SBC A, (HL)
///
/// A <- A - (HL) - CY
///
/// Subtracts the contents of register pair HL and CY to the contents of
/// register A and stores the results in register A.
pub fn sub_with_carry_hl_location_from_a(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.read_hl_location()?;

    emulator.subtract_from_a(value, true);

    Ok(())
}

pub const SUB_WITH_CARRY_HL_LOCATION_FROM_A: Instruction = Instruction {
    name: "SBC A, (HL)",
    op: sub_with_carry_hl_location_from_a,
    pattern: "10 011 110",
    requires_prefix: false,
};

/// SBC A, n
///
/// A <- A - n - CY
///
/// Subtracts 8-bit immediate operand n and CY to the contents of register A and
/// stores the results in register A.
pub fn sub_with_carry_immediate_n_from_a(emulator: &mut Emulator, _: u8) -> OpResult {
    let n = emulator.read_immediate_n()?;

    emulator.subtract_from_a(n, true);

    Ok(())
}

pub const SUB_WITH_CARRY_IMMEDIATE_N_FROM_A: Instruction = Instruction {
    name: "SBC A, n",
    op: sub_with_carry_immediate_n_from_a,
    pattern: "11 011 110",
    requires_prefix: false,
};

/// SBC A, r
///
/// A <- A - r - CY
///
/// Subtracts the contents of register r and CY to those of register A and
/// stores the results in register A.
pub fn sub_with_carry_register_from_a(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register = opcode.parse_register(0b00000111)?;

    let value = emulator.register(&register);

    emulator.subtract_from_a(value, true);

    Ok(())
}

pub const SUB_WITH_CARRY_REGISTER_FROM_A: Instruction = Instruction {
    name: "SBC A, r",
    op: sub_with_carry_register_from_a,
    pattern: "10 011 rrr",
    requires_prefix: false,
};

pub fn add_arithmetic_instructions(emulator: &mut Emulator) {
    emulator.add_instruction(ADD_HL_LOCATION_TO_A);
    emulator.add_instruction(ADD_IMMEDIATE_E_TO_SP);
    emulator.add_instruction(ADD_IMMEDIATE_N_TO_A);
    emulator.add_instruction(ADD_REGISTER_PAIR_TO_HL);
    emulator.add_instruction(ADD_REGISTER_TO_A);
    emulator.add_instruction(ADD_WITH_CARRY_HL_LOCATION_TO_A);
    emulator.add_instruction(ADD_WITH_CARRY_IMMEDIATE_N_TO_A);
    emulator.add_instruction(ADD_WITH_CARRY_REGISTER_TO_A);
    emulator.add_instruction(COMPARE_HL_LOCATION_WITH_A);
    emulator.add_instruction(COMPARE_IMMEDIATE_N_WITH_A);
    emulator.add_instruction(COMPARE_REGISTER_WITH_A);
    emulator.add_instruction(DEC_HL_LOCATION);
    emulator.add_instruction(DEC_REGISTER);
    emulator.add_instruction(DEC_REGISTER_PAIR);
    emulator.add_instruction(INC_HL_LOCATION);
    emulator.add_instruction(INC_REGISTER);
    emulator.add_instruction(INC_REGISTER_PAIR);
    emulator.add_instruction(SUB_HL_LOCATION_FROM_A);
    emulator.add_instruction(SUB_IMMEDIATE_N_FROM_A);
    emulator.add_instruction(SUB_REGISTER_FROM_A);
    emulator.add_instruction(SUB_WITH_CARRY_HL_LOCATION_FROM_A);
    emulator.add_instruction(SUB_WITH_CARRY_IMMEDIATE_N_FROM_A);
    emulator.add_instruction(SUB_WITH_CARRY_REGISTER_FROM_A);
}

// #[cfg(test)]
// mod tests {
//     mod add_hl_location_to_a {
//         use std::collections::HashMap;

//         use common::opcode::Opcode;

//         use crate::{
//             emulator::{test_emulator, Emulator},
//             instruction::arithmetic_instructions::ADD_HL_LOCATION_TO_A_PATTERN,
//             register::Register,
//         };

//         fn run_operation() -> Emulator {
//             // Only one variation
//             let opcode = Opcode::variations(ADD_HL_LOCATION_TO_A_PATTERN)[0];

//             let h = 0x02u8;
//             let l = 0x03u8;
//             let hl = u16::from_le_bytes([l, h]);

//             let mut memory_state = HashMap::<u16, u8>::new();
//             memory_state.insert(hl, 0x04u8);

//             let mut emulator = test_emulator(memory_state);

//             emulator.set_register(Register::A, 0x01u8);
//             emulator.set_register(Register::H, h);
//             emulator.set_register(Register::L, l);

//             emulator.process_opcode(opcode);

//             emulator
//         }

//         #[test]
//         fn cycles() {
//             let emulator = run_operation();

//             assert_eq!(emulator.cycles(), 2);
//         }

//         #[test]
//         fn operation() {
//             let emulator = run_operation();

//             assert_eq!(emulator.register(&Register::A), 0x05u8);
//         }

//         #[test]
//         fn variations() {
//             assert_eq!(Opcode::variations(ADD_HL_LOCATION_TO_A_PATTERN).len(), 1);
//         }
//     }

//     mod add_immediate_n_to_a {
//         use std::collections::HashMap;

//         use common::opcode::Opcode;

//         use crate::{
//             addresses::PROGRAM_COUNTER_START,
//             emulator::{test_emulator, Emulator},
//             instruction::arithmetic_instructions::ADD_IMMEDIATE_N_TO_A_PATTERN,
//             register::Register,
//         };

//         fn run_operation() -> Emulator {
//             // Only one variation
//             let opcode = Opcode::variations(ADD_IMMEDIATE_N_TO_A_PATTERN)[0];

//             let mut memory_state = HashMap::<u16, u8>::new();
//             memory_state.insert(PROGRAM_COUNTER_START, 0x03);

//             let mut emulator = test_emulator(memory_state);

//             emulator.set_register(Register::A, 0x01u8);

//             emulator.process_opcode(opcode);

//             emulator
//         }

//         #[test]
//         fn cycles() {
//             let emulator = run_operation();

//             assert_eq!(emulator.cycles(), 2);
//         }

//         #[test]
//         fn operation() {
//             let emulator = run_operation();

//             assert_eq!(emulator.register(&Register::A), 0x04u8);
//         }

//         #[test]
//         fn variations() {
//             assert_eq!(Opcode::variations(ADD_IMMEDIATE_N_TO_A_PATTERN).len(), 1);
//         }
//     }

//     mod add_register_to_a {
//         use common::opcode::Opcode;

//         use crate::{
//             emulator::Emulator,
//             flag::Flag,
//             instruction::arithmetic_instructions::ADD_REGISTER_TO_A_PATTERN,
//             register::Register,
//         };

//         fn run_operation(opcode: u8) -> Emulator {
//             let mut emulator = Emulator::new();

//             emulator.set_register(Register::A, 1u8);
//             emulator.set_register(Register::B, 1u8);
//             emulator.set_register(Register::C, 2u8);
//             emulator.set_register(Register::D, 3u8);
//             emulator.set_register(Register::E, 4u8);
//             emulator.set_register(Register::H, 6u8);
//             emulator.set_register(Register::L, 7u8);

//             emulator.process_opcode(opcode);

//             emulator
//         }

//         #[test]
//         fn cycles() {
//             for opcode in Opcode::variations(ADD_REGISTER_TO_A_PATTERN) {
//                 let emulator = run_operation(opcode);

//                 assert_eq!(emulator.cycles(), 1);
//             }
//         }

//         #[test]
//         fn operation() {
//             // Test each variation
//             for opcode in Opcode::variations(ADD_REGISTER_TO_A_PATTERN) {
//                 let emulator = run_operation(opcode);

//                 let source_register = opcode.parse_register(0b00000111u8).unwrap();

//                 let expected_value = if source_register == Register::A {
//                     2u8
//                 } else {
//                     emulator.register(&source_register) + 1u8
//                 };

//                 // It should add the values of the A and source register
//                 assert_eq!(
//                     emulator.register(&Register::A),
//                     expected_value,
//                     "Failed for source {:?}",
//                     source_register
//                 );
//                 assert_eq!(
//                     emulator.flag(Flag::CY),
//                     false,
//                     "Failed for source {:?}",
//                     source_register
//                 );
//                 assert_eq!(
//                     emulator.flag(Flag::H),
//                     false,
//                     "Failed for source {:?}",
//                     source_register
//                 );
//                 assert_eq!(
//                     emulator.flag(Flag::N),
//                     false,
//                     "Failed for source {:?}",
//                     source_register
//                 );
//                 assert_eq!(
//                     emulator.flag(Flag::Z),
//                     false,
//                     "Failed for source {:?}",
//                     source_register
//                 );
//             }
//         }

//         #[test]
//         fn variations() {
//             assert_eq!(Opcode::variations(ADD_REGISTER_TO_A_PATTERN).len(), 7);
//         }
//     }

//     mod add_with_carry_hl_location_to_a {
//         use std::collections::HashMap;

//         use common::opcode::Opcode;

//         use crate::{
//             emulator::{test_emulator, Emulator},
//             flag::Flag,
//             instruction::arithmetic_instructions::ADD_WITH_CARRY_HL_LOCATION_TO_A_PATTERN,
//             register::Register,
//         };

//         fn run_operation() -> Emulator {
//             // Only one variation
//             let opcode = Opcode::variations(ADD_WITH_CARRY_HL_LOCATION_TO_A_PATTERN)[0];

//             let h = 0x02u8;
//             let l = 0x03u8;
//             let hl = u16::from_le_bytes([l, h]);

//             let mut memory_state = HashMap::<u16, u8>::new();
//             memory_state.insert(hl, 0x04u8);

//             let mut emulator = test_emulator(memory_state);

//             emulator.set_flag(Flag::CY, true);

//             emulator.set_register(Register::A, 0x01u8);
//             emulator.set_register(Register::H, h);
//             emulator.set_register(Register::L, l);

//             emulator.process_opcode(opcode);

//             emulator
//         }

//         #[test]
//         fn cycles() {
//             let emulator = run_operation();

//             assert_eq!(emulator.cycles(), 2);
//         }

//         #[test]
//         fn operation() {
//             let emulator = run_operation();

//             assert_eq!(emulator.register(&Register::A), 0x06u8);
//         }

//         #[test]
//         fn variations() {
//             assert_eq!(
//                 Opcode::variations(ADD_WITH_CARRY_HL_LOCATION_TO_A_PATTERN).len(),
//                 1
//             );
//         }
//     }

//     mod add_with_carry_immediate_n_to_a {
//         use std::collections::HashMap;

//         use common::opcode::Opcode;

//         use crate::{
//             addresses::PROGRAM_COUNTER_START,
//             emulator::{test_emulator, Emulator},
//             flag::Flag,
//             instruction::arithmetic_instructions::ADD_WITH_CARRY_IMMEDIATE_N_TO_A_PATTERN,
//             register::Register,
//         };

//         fn run_operation() -> Emulator {
//             // Only one variation
//             let opcode = Opcode::variations(ADD_WITH_CARRY_IMMEDIATE_N_TO_A_PATTERN)[0];

//             let mut memory_state = HashMap::<u16, u8>::new();
//             memory_state.insert(PROGRAM_COUNTER_START, 0x03);

//             let mut emulator = test_emulator(memory_state);

//             emulator.set_flag(Flag::CY, true);

//             emulator.set_register(Register::A, 0x01u8);

//             emulator.process_opcode(opcode);

//             emulator
//         }

//         #[test]
//         fn cycles() {
//             let emulator = run_operation();

//             assert_eq!(emulator.cycles(), 2);
//         }

//         #[test]
//         fn operation() {
//             let emulator = run_operation();

//             assert_eq!(emulator.register(&Register::A), 0x05u8);
//         }

//         #[test]
//         fn variations() {
//             assert_eq!(
//                 Opcode::variations(ADD_WITH_CARRY_IMMEDIATE_N_TO_A_PATTERN).len(),
//                 1
//             );
//         }
//     }

//     mod add_with_carry_register_to_a {
//         use common::opcode::Opcode;

//         use crate::{
//             emulator::Emulator,
//             flag::Flag,
//             instruction::arithmetic_instructions::ADD_WITH_CARRY_REGISTER_TO_A_PATTERN,
//             opcode::Opcode,
//             register::Register,
//         };

//         fn run_operation(opcode: u8) -> Emulator {
//             let mut emulator = Emulator::new();

//             emulator.set_flag(Flag::CY, true);

//             emulator.set_register(Register::A, 1u8);
//             emulator.set_register(Register::B, 1u8);
//             emulator.set_register(Register::C, 2u8);
//             emulator.set_register(Register::D, 3u8);
//             emulator.set_register(Register::E, 4u8);
//             emulator.set_register(Register::H, 6u8);
//             emulator.set_register(Register::L, 7u8);

//             emulator.process_opcode(opcode);

//             emulator
//         }

//         #[test]
//         fn cycles() {
//             for opcode in Opcode::variations(ADD_WITH_CARRY_REGISTER_TO_A_PATTERN) {
//                 let emulator = run_operation(opcode);

//                 assert_eq!(emulator.cycles(), 1);
//             }
//         }

//         #[test]
//         fn operation() {
//             // Test each variation
//             for opcode in Opcode::variations(ADD_WITH_CARRY_REGISTER_TO_A_PATTERN) {
//                 let emulator = run_operation(opcode);

//                 let source_register = opcode.parse_register(0b00_000_111u8).unwrap();

//                 let expected_value = if source_register == Register::A {
//                     2u8
//                 } else {
//                     emulator.register(&source_register) + 1u8
//                 };

//                 // It should add the values of the A and source register
//                 assert_eq!(
//                     emulator.register(&Register::A),
//                     expected_value,
//                     "Failed for source {:?}",
//                     source_register
//                 );
//                 assert_eq!(
//                     emulator.flag(Flag::CY),
//                     false,
//                     "Failed for source {:?}",
//                     source_register
//                 );
//                 assert_eq!(
//                     emulator.flag(Flag::H),
//                     false,
//                     "Failed for source {:?}",
//                     source_register
//                 );
//                 assert_eq!(
//                     emulator.flag(Flag::N),
//                     false,
//                     "Failed for source {:?}",
//                     source_register
//                 );
//                 assert_eq!(
//                     emulator.flag(Flag::Z),
//                     false,
//                     "Failed for source {:?}",
//                     source_register
//                 );
//             }
//         }

//         #[test]
//         fn variations() {
//             assert_eq!(
//                 Opcode::variations(ADD_WITH_CARRY_REGISTER_TO_A_PATTERN).len(),
//                 7
//             );
//         }
//     }
// }
