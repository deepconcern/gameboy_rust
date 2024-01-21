use crate::{
    addresses::PORT_REGISTER_START,
    emulator::Emulator,
    instruction::{Instruction, OpResult},
    opcode::Opcode,
    register::{Register, RegisterPair},
};

/// LD (BC), A
/// 
/// (BC) <- A
/// 
/// Load the contents of register A into the memory specified by the contents
/// register pair BC.
pub fn load_a_into_bc_location(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.a();

    let location = emulator.register_pair(&RegisterPair::Bc);

    emulator.write(location, value)?;

    Ok(())
}

pub const LOAD_A_INTO_BC_LOCATION: Instruction = Instruction {
    name: "LD (BC), A",
    op: load_a_into_bc_location,
    pattern: "00 000 010",
    requires_prefix: false,
};

/// LD (C), A
/// 
/// (FF00H+C) <- A
/// 
/// Load the contents of register A into the internal RAM, port register, or
/// mode register at the address in the range FF00h-FFFFh specified by register
/// C.
pub fn load_a_into_c_location(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.a();

    let location_offset = emulator.register(&Register::C) as u16;

    emulator.write(PORT_REGISTER_START + location_offset, value)?;

    Ok(())
}

pub const LOAD_A_INTO_C_LOCATION: Instruction = Instruction {
    name: "LD (C), A",
    op: load_a_into_c_location,
    pattern: "11 100 010",
    requires_prefix: false,
};

/// LD (DE), A
/// 
/// (DE) <- A
/// 
/// Load the contents of register A into the memory specified by the contents
/// register pair DE.
pub fn load_a_into_de_location(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.a();

    let location = emulator.register_pair(&RegisterPair::De);

    emulator.write(location, value)?;

    Ok(())
}

pub const LOAD_A_INTO_DE_LOCATION: Instruction = Instruction {
    name: "LD (DE), A",
    op: load_a_into_de_location,
    pattern: "00 010 010",
    requires_prefix: false,
};

/// LD (HLD), A
/// 
/// (HL) <- A
/// HL <- HL+1
/// 
/// Load the contents of register A into the memory specified by the contents
/// register pair HL, and then decrement the contents of register pair HL.
pub fn load_a_into_hl_location_dec(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.a();

    emulator.write_hl_location(value)?;

    emulator.set_register_pair(RegisterPair::Hl, emulator.register_pair(&RegisterPair::Hl) - 1);

    Ok(())
}

pub const LOAD_A_INTO_HL_LOCATION_DEC: Instruction = Instruction {
    name: "LD (HLD), A",
    op: load_a_into_hl_location_dec,
    pattern: "00 110 010",
    requires_prefix: false,
};

/// LD (HLI), A
/// 
/// (HL) <- A
/// HL <- HL+1
/// 
/// Load the contents of register A into the memory specified by the contents
/// register pair HL, and then increment the contents of register pair HL.
pub fn load_a_into_hl_location_inc(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.a();

    emulator.write_hl_location(value)?;

    emulator.set_register_pair(RegisterPair::Hl, emulator.register_pair(&RegisterPair::Hl) + 1);

    Ok(())
}

pub const LOAD_A_INTO_HL_LOCATION_INC: Instruction = Instruction {
    name: "LD (HLI), A",
    op: load_a_into_hl_location_inc,
    pattern: "00 100 010",
    requires_prefix: false,
};

/// LD (n), A
/// 
/// (FF00H+n) <- A
/// 
/// Load the contents of register A into the internal RAM, port register, or
/// mode register at the address in the range FF00h-FFFFh specified by 8-bit
/// immediate operand n.
pub fn load_a_into_immediate_n_location(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.a();

    let location_offset = emulator.read_immediate_n()? as u16;

    emulator.write(PORT_REGISTER_START + location_offset, value)?;

    Ok(())
}

pub const LOAD_A_INTO_IMMEDIATE_N_LOCATION: Instruction = Instruction {
    name: "LD (n), A",
    op: load_a_into_immediate_n_location,
    pattern: "11 100 000",
    requires_prefix: false,
};

/// LD (n), A
/// 
/// (nn) <- A
/// 
/// Load the contents of register A into the memory specified by 16-bit
/// immediate operand nn.
pub fn load_a_into_immediate_nn_location(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.a();

    let nn = emulator.read_immediate_nn()?;

    emulator.write(nn, value)?;

    Ok(())
}

pub const LOAD_A_INTO_IMMEDIATE_NN_LOCATION: Instruction = Instruction {
    name: "LD (nn), A",
    op: load_a_into_immediate_nn_location,
    pattern: "11 101 010",
    requires_prefix: false,
};

/// LD A, (BC)
/// 
/// A <- (BC)
/// 
/// Load the contents of memory specified by the contents of register pair BC
/// into register A.
pub fn load_bc_location_into_a(emulator: &mut Emulator, _: u8) -> OpResult {
    let location = emulator.register_pair(&RegisterPair::Bc);

    let value = emulator.read(location)?;

    emulator.set_a(value);

    Ok(())
}

pub const LOAD_BC_LOCATION_INTO_A: Instruction = Instruction {
    name: "LD A, (BC)",
    op: load_bc_location_into_a,
    pattern: "00 001 010",
    requires_prefix: false,
};

/// LD A, (C)
/// 
/// A <- (FF00H+C)
/// 
/// Load the contents of the internal RAM, port register, or mode register at
/// the address in the range FF00h-FFFFh specified by register C into register
/// A.
pub fn load_c_location_into_a(emulator: &mut Emulator, _: u8) -> OpResult {
    let location_offset = emulator.register(&Register::C) as u16;

    let value = emulator.read(PORT_REGISTER_START + location_offset)?;

    emulator.set_a(value);

    Ok(())
}

pub const LOAD_C_LOCATION_INTO_A: Instruction = Instruction {
    name: "LD A, (C)",
    op: load_c_location_into_a,
    pattern: "11 110 010",
    requires_prefix: false,
};

/// LD A, (DE)
/// 
/// A <- (DE)
/// 
/// Load the contents of memory specified by the contents of register pair DE
/// into register A.
pub fn load_de_location_into_a(emulator: &mut Emulator, _: u8) -> OpResult {
    let location = emulator.register_pair(&RegisterPair::De);

    let value = emulator.read(location)?;

    emulator.set_a(value);

    Ok(())
}

pub const LOAD_DE_LOCATION_INTO_A: Instruction = Instruction {
    name: "LD A, (DE)",
    op: load_de_location_into_a,
    pattern: "00 011 010",
    requires_prefix: false,
};

/// LD SP, HL
/// 
/// SP <- HL
/// 
/// Load the contents of register pair HL into the stack pointer.
pub fn load_hl_into_sp(emulator: &mut Emulator, _: u8) -> OpResult {
    let value = emulator.register_pair(&RegisterPair::Hl);

    emulator.set_stack_pointer(value);

    Ok(())
}

pub const LOAD_HL_INTO_SP: Instruction = Instruction {
    name: "LD A, (HLD)",
    op: load_hl_into_sp,
    pattern: "11 111 001",
    requires_prefix: false,
};

/// LD A, (HLD)
/// 
/// A <- (HL)
/// HL <- HL+1
/// 
/// Load the contents of the memory specified by the contents register pair HL
/// into register A, and then decrement the contents of register pair HL.
pub fn load_hl_location_dec_into_a(emulator: &mut Emulator, _: u8) -> OpResult {
    let location = emulator.register_pair(&RegisterPair::Hl);

    let value = emulator.read(location)?;

    emulator.set_a(value);

    emulator.set_register_pair(RegisterPair::Hl, location - 1);

    Ok(())
}

pub const LOAD_HL_LOCATION_DEC_INTO_A: Instruction = Instruction {
    name: "LD A, (HLI)",
    op: load_hl_location_dec_into_a,
    pattern: "00 111 010",
    requires_prefix: false,
};

/// LD A, (HLI)
/// 
/// A <- (HL)
/// HL <- HL+1
/// 
/// Load the contents of the memory specified by the contents register pair HL
/// into register A, and then increment the contents of register pair HL.
pub fn load_hl_location_inc_into_a(emulator: &mut Emulator, _: u8) -> OpResult {
    let location = emulator.register_pair(&RegisterPair::Hl);

    let value = emulator.read(location)?;

    emulator.set_a(value);

    emulator.set_register_pair(RegisterPair::Hl, location + 1);

    Ok(())
}

pub const LOAD_HL_LOCATION_INC_INTO_A: Instruction = Instruction {
    name: "LD r, (HL)",
    op: load_hl_location_inc_into_a,
    pattern: "00 101 010",
    requires_prefix: false,
};

/// LD r, (HL)
/// 
/// r <- (HL)
/// 
/// Load the contents of memory specified by the contents of register pair HL
/// into register r.
pub fn load_hl_location_into_register(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let location = emulator.register_pair(&RegisterPair::Hl);

    let value = emulator.read(location)?;

    let destination_register = opcode.parse_register(0b00111000)?;

    emulator.set_register(destination_register, value);

    Ok(())
}

pub const LOAD_HL_LOCATION_INTO_REGISTER: Instruction = Instruction {
    name: "LD SP, HL",
    op: load_hl_location_into_register,
    pattern: "01 rrr 110",
    requires_prefix: false,
};

/// LD (HL), n
///
/// (HL) <- n
/// 
/// Load the 8-bit immediate operand n into into the location of memory
/// specified by the contents of register pair HL.
pub fn load_immediate_n_into_hl_location(emulator: &mut Emulator, _: u8) -> OpResult {
    let n = emulator.read_immediate_n()?;

    emulator.write_hl_location(n)?;

    Ok(())
}

pub const LOAD_IMMEDIATE_N_INTO_HL_LOCATION: Instruction = Instruction {
    name: "LD (HL), n",
    op: load_immediate_n_into_hl_location,
    pattern: "00 110 110",
    requires_prefix: false,
};

/// LD r, n
///
/// r <- n
/// 
/// Load the 8-bit immediate operand n into register r.
pub fn load_immediate_n_into_register(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let n = emulator.read_immediate_n()?;

    let destination_register = opcode.parse_register(0b00111000)?;

    emulator.set_register(destination_register, n);

    Ok(())
}

pub const LOAD_IMMEDIATE_N_INTO_REGISTER: Instruction = Instruction {
    name: "LD r, n",
    op: load_immediate_n_into_register,
    pattern: "00 rrr 110",
    requires_prefix: false,
};

/// LD A, (n)
/// 
/// A <- (FF00H+n)
/// 
/// Load the contents of the internal RAM, port register, or mode register at
/// the address in the range FF00h-FFFFh specified by 8-bit immediate operand n
/// into register A.
pub fn load_immediate_n_location_into_a(emulator: &mut Emulator, _: u8) -> OpResult {
    let location_offset = emulator.read_immediate_n()? as u16;

    let value = emulator.read(PORT_REGISTER_START + location_offset)?;

    emulator.set_a(value);

    Ok(())
}

pub const LOAD_IMMEDIATE_N_LOCATION_INTO_A: Instruction = Instruction {
    name: "LD A, (n)",
    op: load_immediate_n_location_into_a,
    pattern: "11 110 000",
    requires_prefix: false,
};

/// LD ss, nn
/// 
/// ss <- nn
/// 
/// Load the contents of 16-bit immediate operand nn into register pair ss.
pub fn load_immediate_nn_into_register_pair(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register_pair = opcode.parse_register_pair(0b00_110_000)?;
    let nn = emulator.read_immediate_nn()?;

    emulator.set_register_pair(register_pair, nn);

    Ok(())
}

pub const LOAD_IMMEDIATE_NN_INTO_REGISTER_PAIR: Instruction = Instruction {
    name: "LD A, (nn)",
    op: load_immediate_nn_into_register_pair,
    pattern: "00 ss0 001",
    requires_prefix: false,
};

/// LD A, (nn)
/// 
/// A <- (nn)
/// 
/// Load the contents of the memory specified by 16-bit immediate operand nn
/// into register A.
pub fn load_immediate_nn_location_into_a(emulator: &mut Emulator, _: u8) -> OpResult {
    let nn = emulator.read_immediate_nn()?;

    let value = emulator.read(nn)?;

    emulator.set_a(value);

    Ok(())
}

pub const LOAD_IMMEDIATE_NN_LOCATION_INTO_A: Instruction = Instruction {
    name: "LD qq, nn",
    op: load_immediate_nn_location_into_a,
    pattern: "11 111 010",
    requires_prefix: false,
};

/// LD (HL), r
/// 
/// (HL) <- r
/// 
/// Load the contents of register r into the location of memory specified by the
/// contents of register pair HL.
pub fn load_register_into_hl_location(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let source_register = opcode.parse_register(0b00000111)?;

    let value = emulator.register(&source_register);

    emulator.write_hl_location(value)?;

    Ok(())
}

pub const LOAD_REGISTER_INTO_HL_LOCATION: Instruction = Instruction {
    name: "LD (HL), r",
    op: load_register_into_hl_location,
    pattern: "01 110 rrr",
    requires_prefix: false,
};

/// LD r, r'
///
/// r <- 'r
///
/// Load the contents of register r' into register r
pub fn load_register_into_register(
    emulator: &mut Emulator,
    opcode: u8,
) -> OpResult {
    let source_register = opcode.parse_register(0b00000111)?;
    let destination_register = opcode.parse_register(0b00111000)?;

    let source_value = emulator.register(&source_register);

    emulator.set_register(destination_register, source_value);

    Ok(())
}

pub const LOAD_REGISTER_INTO_REGISTER: Instruction = Instruction {
    name: "LD r, r",
    op: load_register_into_register,
    pattern: "01 rrr qqq",
    requires_prefix: false,
};

/// LDHL SP, e
/// 
/// HL <- SP + e
/// 
/// Load the contents of the stack pointer and 8-bit immediate operand e into
/// register pair HL.
pub fn load_sp_and_immediate_e_into_hl(emulator: &mut Emulator, _: u8) -> OpResult {
    let e = emulator.read_immediate_e()?;

    let value = emulator.add_signed(emulator.stack_pointer(), e, false);

    emulator.set_register_pair(RegisterPair::Hl, value as u16);

    Ok(())
}

pub const LOAD_SP_AND_IMMEDIATE_E_INTO_HL: Instruction = Instruction {
    name: "LDHL SP, e",
    op: load_sp_and_immediate_e_into_hl,
    pattern: "11 111 000",
    requires_prefix: false,
};

/// LD (nn), SP
/// 
/// (nn) <- SPL
/// (nn + 1) <- SPH
/// 
/// Load the contents of the stack pointer into memory specified by 16-bit
/// immediate operand nn.
pub fn load_sp_into_nn_location(emulator: &mut Emulator, _: u8) -> OpResult {
    let nn = emulator.read_immediate_nn()?;

    let [low_value, high_value] = emulator.stack_pointer().to_le_bytes();

    emulator.write(nn, low_value)?;
    emulator.write(nn.wrapping_add(1), high_value)?;

    Ok(())
}

pub const LOAD_SP_INTO_NN_LOCATION: Instruction = Instruction {
    name: "LD (nn), SP",
    op: load_sp_into_nn_location,
    pattern: "00 001 000",
    requires_prefix: false,
};

/// POP qq
/// 
/// qqL <- (SP)
/// qqH <- (SP + 1)
/// SP <- SP + 2
/// 
/// Pops contents from the memory stack and into register pair qq.
pub fn pop_register_pair(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register_pair = opcode.parse_register_pair(0b00_110_000)?;

    let low = emulator.read(emulator.stack_pointer())?;
    let high = emulator.read(emulator.stack_pointer() + 1)?;

    emulator.set_register_pair(register_pair, u16::from_le_bytes([low, high]));

    emulator.set_stack_pointer(emulator.stack_pointer() + 2);

    Ok(())
}

pub const POP_REGISTER_PAIR: Instruction = Instruction {
    name: "POP ss",
    op: pop_register_pair,
    pattern: "11 ss0 001",
    requires_prefix: false,
};

/// PUSH qq
/// 
/// (SP - 1) <- qqH
/// (SP - 2) <- qqL
/// SP <- SP - 2
/// 
/// Pushes the contents of register pair qq onto the memory stack.
pub fn push_register_pair(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let register_pair = opcode.parse_register_pair(0b00_110_000)?;

    let [low, high] = emulator.register_pair(&register_pair).to_le_bytes();

    emulator.write(emulator.stack_pointer() - 1, high)?;
    emulator.write(emulator.stack_pointer() - 2, low)?;

    emulator.set_stack_pointer(emulator.stack_pointer() - 2);

    Ok(())
}

pub const PUSH_REGISTER_PAIR: Instruction = Instruction {
    name: "PUSH ss",
    op: push_register_pair,
    pattern: "11 ss0 101",
    requires_prefix: false,
};

pub fn add_loading_instructions(emulator: &mut Emulator) {
    emulator.add_instruction(LOAD_A_INTO_BC_LOCATION);
    emulator.add_instruction(LOAD_A_INTO_C_LOCATION);
    emulator.add_instruction(LOAD_A_INTO_DE_LOCATION);
    emulator.add_instruction(LOAD_A_INTO_HL_LOCATION_DEC);
    emulator.add_instruction(LOAD_A_INTO_HL_LOCATION_INC);
    emulator.add_instruction(LOAD_A_INTO_IMMEDIATE_N_LOCATION);
    emulator.add_instruction(LOAD_A_INTO_IMMEDIATE_NN_LOCATION);
    emulator.add_instruction(LOAD_BC_LOCATION_INTO_A);
    emulator.add_instruction(LOAD_C_LOCATION_INTO_A);
    emulator.add_instruction(LOAD_DE_LOCATION_INTO_A);
    emulator.add_instruction(LOAD_HL_LOCATION_DEC_INTO_A);
    emulator.add_instruction(LOAD_HL_LOCATION_INC_INTO_A);
    emulator.add_instruction(LOAD_HL_LOCATION_INTO_REGISTER);
    emulator.add_instruction(LOAD_HL_INTO_SP);
    emulator.add_instruction(LOAD_IMMEDIATE_N_INTO_HL_LOCATION);
    emulator.add_instruction(LOAD_IMMEDIATE_N_INTO_REGISTER);
    emulator.add_instruction(LOAD_IMMEDIATE_N_LOCATION_INTO_A);
    emulator.add_instruction(LOAD_IMMEDIATE_NN_LOCATION_INTO_A);
    emulator.add_instruction(LOAD_IMMEDIATE_NN_INTO_REGISTER_PAIR);
    emulator.add_instruction(LOAD_REGISTER_INTO_HL_LOCATION);
    emulator.add_instruction(LOAD_REGISTER_INTO_REGISTER);
    emulator.add_instruction(LOAD_SP_AND_IMMEDIATE_E_INTO_HL);
    emulator.add_instruction(LOAD_SP_INTO_NN_LOCATION);
    emulator.add_instruction(POP_REGISTER_PAIR);
    emulator.add_instruction(PUSH_REGISTER_PAIR);
}

// #[cfg(test)]
// mod tests {
//     mod load_a_into_c_location {
//         use std::collections::HashMap;

//         use common::opcode::Opcode;

//         use crate::addresses::PORT_REGISTER_START;
//         use crate::emulator::{Emulator, test_emulator};
//         use crate::instruction::loading_instructions::LOAD_A_INTO_C_LOCATION_PATTERN;
//         use crate::register::Register;

//         fn run_operation() -> Emulator {
//             let opcode = Opcode::variations(LOAD_A_INTO_C_LOCATION_PATTERN)[0];

//             let mut memory_state = HashMap::<u16, u8>::new();
//             memory_state.insert( PORT_REGISTER_START + 0x0004u16, 0x00u8);

//             let mut emulator = test_emulator(memory_state);

//             emulator.set_a(0x03u8);
//             emulator.set_register(Register::C, 0x04u8);

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

//             assert_eq!(emulator.memory_location(PORT_REGISTER_START + 0x0004u16), 0x03u8);
//         }

//         #[test]
//         fn variations() {
//             assert_eq!(Opcode::variations(LOAD_A_INTO_C_LOCATION_PATTERN).len(), 1);
//         }
//     }

//     mod load_a_into_immediate_n_location {
//         use std::collections::HashMap;

//         use common::opcode::Opcode;

//         use crate::addresses::PORT_REGISTER_START;
//         use crate::emulator::{Emulator, test_emulator};
//         use crate::instruction::loading_instructions::LOAD_A_INTO_C_LOCATION_PATTERN;
//         use crate::register::Register;

//         fn run_operation() -> Emulator {
//             let opcode = Opcode::variations(LOAD_A_INTO_C_LOCATION_PATTERN)[0];

//             let mut memory_state = HashMap::<u16, u8>::new();
//             memory_state.insert( PORT_REGISTER_START + 0x0004u16, 0x00u8);

//             let mut emulator = test_emulator(memory_state);

//             emulator.set_a(0x03u8);
//             emulator.set_register(Register::C, 0x04u8);

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

//             assert_eq!(emulator.memory_location(PORT_REGISTER_START + 0x0004u16), 0x03u8);
//         }

//         #[test]
//         fn variations() {
//             assert_eq!(Opcode::variations(LOAD_A_INTO_C_LOCATION_PATTERN).len(), 1);
//         }
//     }

//     mod load_bc_location_into_a {
//         use std::collections::HashMap;

//         use common::opcode::Opcode;

//         use crate::emulator::{Emulator, test_emulator};
//         use crate::instruction::loading_instructions::LOAD_BC_LOCATION_INTO_A_PATTERN;
//         use crate::register::Register;

//         fn run_operation() -> Emulator {
//             let opcode = Opcode::variations(LOAD_BC_LOCATION_INTO_A_PATTERN)[0];

//             let b = 0x02u8;
//             let c = 0x03u8;
//             let bc = u16::from_le_bytes([c, b]);

//             let mut memory_state = HashMap::<u16, u8>::new();
//             memory_state.insert(bc, 0x03u8);

//             let mut emulator = test_emulator(memory_state);

//             emulator.set_register(Register::B, b);
//             emulator.set_register(Register::C, c);

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

//             assert_eq!(emulator.a(), 0x03u8);
//         }

//         #[test]
//         fn variations() {
//             assert_eq!(Opcode::variations(LOAD_BC_LOCATION_INTO_A_PATTERN).len(), 1);
//         }
//     }

//     mod load_c_location_into_a {
//         use std::collections::HashMap;

//         use common::opcode::Opcode;

//         use crate::addresses::PORT_REGISTER_START;
//         use crate::emulator::{Emulator, test_emulator};
//         use crate::instruction::loading_instructions::LOAD_C_LOCATION_INTO_A_PATTERN;
//         use crate::register::Register;

//         fn run_operation() -> Emulator {
//             let opcode = Opcode::variations(LOAD_C_LOCATION_INTO_A_PATTERN)[0];

//             let mut memory_state = HashMap::<u16, u8>::new();
//             memory_state.insert( PORT_REGISTER_START + 0x0004u16, 0x03u8);

//             let mut emulator = test_emulator(memory_state);

//             emulator.set_register(Register::C, 0x04u8);

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

//             assert_eq!(emulator.a(), 0x03u8);
//         }

//         #[test]
//         fn variations() {
//             assert_eq!(Opcode::variations(LOAD_C_LOCATION_INTO_A_PATTERN).len(), 1);
//         }
//     }

//     mod load_de_location_into_a {
//         use std::collections::HashMap;

//         use common::opcode::Opcode;

//         use crate::emulator::{Emulator, test_emulator};
//         use crate::instruction::loading_instructions::LOAD_DE_LOCATION_INTO_A_PATTERN;
//         use crate::register::Register;

//         fn run_operation() -> Emulator {
//             let opcode = Opcode::variations(LOAD_DE_LOCATION_INTO_A_PATTERN)[0];

//             let d = 0x02u8;
//             let e = 0x03u8;
//             let de = u16::from_le_bytes([e, d]);

//             let mut memory_state = HashMap::<u16, u8>::new();
//             memory_state.insert(de, 0x03u8);

//             let mut emulator = test_emulator(memory_state);

//             emulator.set_register(Register::D, d);
//             emulator.set_register(Register::E, e);

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

//             assert_eq!(emulator.a(), 0x03u8);
//         }

//         #[test]
//         fn variations() {
//             assert_eq!(Opcode::variations(LOAD_DE_LOCATION_INTO_A_PATTERN).len(), 1);
//         }
//     }

//     mod load_hl_location_into_register {
//         use std::collections::HashMap;

//         use common::opcode::Opcode;

//         use crate::emulator::{Emulator, test_emulator};
//         use crate::instruction::loading_instructions::LOAD_HL_LOCATION_INTO_REGISTER_PATTERN;
//         use crate::register::Register;

//         fn run_operation(opcode: u8) -> Emulator {
//             let h = 0x02u8;
//             let l = 0x03u8;
//             let hl = u16::from_le_bytes([l, h]);

//             let mut memory_state = HashMap::<u16, u8>::new();
//             memory_state.insert(hl, 0x03u8);

//             let mut emulator = test_emulator(memory_state);

//             emulator.set_register(Register::H, h);
//             emulator.set_register(Register::L, l);

//             emulator.process_opcode(opcode);

//             emulator
//         }

//         #[test]
//         fn cycles() {
//             for opcode in Opcode::variations(LOAD_HL_LOCATION_INTO_REGISTER_PATTERN) {
//                 let emulator = run_operation(opcode);

//                 assert_eq!(emulator.cycles(), 2);
//             }
//         }

//         #[test]
//         fn operation() {
//             // Test each variation
//             for opcode in Opcode::variations(LOAD_HL_LOCATION_INTO_REGISTER_PATTERN) {
//                 let emulator = run_operation(opcode);

//                 let destination_register = opcode.parse_register(0b00111000u8).unwrap();

//                 assert_eq!(
//                     emulator.register(&destination_register),
//                     0x03u8,
//                     "Failed for destination {:?}",
//                     destination_register
//                 );
//             }
//         }

//         #[test]
//         fn variations() {
//             assert_eq!(Opcode::variations(LOAD_HL_LOCATION_INTO_REGISTER_PATTERN).len(), 7);
//         }
//     }

//     mod load_immediate_n_into_hl_location {
//         use std::collections::HashMap;

//         use common::opcode::Opcode;

//         use crate::emulator::{Emulator, test_emulator};
//         use crate::addresses::PROGRAM_COUNTER_START;
//         use crate::instruction::loading_instructions::LOAD_IMMEDIATE_N_INTO_HL_LOCATION_PATTERN;
//         use crate::register::Register;

//         fn run_operation() -> (Emulator, u16) {
//             // Only one variation
//             let opcode = Opcode::variations(LOAD_IMMEDIATE_N_INTO_HL_LOCATION_PATTERN)[0];

//             let h = 0x03u8;
//             let l = 0x03u8;
//             let hl = u16::from_le_bytes([l, h]);

//             let mut memory_state = HashMap::<u16, u8>::new();
//             memory_state.insert(hl, 0x00u8);
//             memory_state.insert(PROGRAM_COUNTER_START, 0x03u8);

//             let mut emulator = test_emulator(memory_state);
            
//             emulator.set_register(Register::H, h);
//             emulator.set_register(Register::L, l);

//             emulator.process_opcode(opcode);

//             (emulator, hl)
//         }

//         #[test]
//         fn cycles() {
//             let (emulator, _) = run_operation();

//             assert_eq!(emulator.cycles(), 3);
//         }

//         #[test]
//         fn operation() {
//             let (emulator, location) = run_operation();

//             assert_eq!(emulator.memory_location(location), 0x03u8);
//         }

//         #[test]
//         fn variations() {
//             assert_eq!(Opcode::variations(LOAD_IMMEDIATE_N_INTO_HL_LOCATION_PATTERN).len(), 1);
//         }
//     }

//     mod load_immediate_n_into_register {
//         use std::collections::HashMap;

//         use common::opcode::Opcode;

//         use crate::instruction::parse_register;
//         use crate::emulator::{Emulator, test_emulator};
//         use crate::addresses::PROGRAM_COUNTER_START;
//         use crate::instruction::loading_instructions::LOAD_IMMEDIATE_N_INTO_REGISTER_PATTERN;

//         fn run_operation(opcode: u8) -> Emulator {
//             let mut memory_state = HashMap::<u16, u8>::new();
//             memory_state.insert(PROGRAM_COUNTER_START, 0x03u8);

//             let mut emulator = test_emulator(memory_state);

//             emulator.process_opcode(opcode);

//             emulator
//         }

//         #[test]
//         fn cycles() {
//             for opcode in Opcode::variations(LOAD_IMMEDIATE_N_INTO_REGISTER_PATTERN) {
//                 let emulator = run_operation(opcode);

//                 assert_eq!(emulator.cycles(), 2);
//             }
//         }

//         #[test]
//         fn operation() {
//             // Test each variation
//             for opcode in Opcode::variations(LOAD_IMMEDIATE_N_INTO_REGISTER_PATTERN) {
//                 let emulator = run_operation(opcode);

//                 let destination_register = opcode.parse_register(0b00111000u8).unwrap();

//                 assert_eq!(
//                     emulator.register(&destination_register),
//                     0x03u8,
//                     "Failed for destination {:?}",
//                     destination_register
//                 );
//             }
//         }

//         #[test]
//         fn variations() {
//             assert_eq!(Opcode::variations(LOAD_IMMEDIATE_N_INTO_REGISTER_PATTERN).len(), 7);
//         }
//     }

//     mod load_register_into_hl_location {
//         use std::collections::HashMap;

//         use common::opcode::Opcode;

//         use crate::instruction::parse_register;
//         use crate::emulator::{Emulator, test_emulator};
//         use crate::instruction::loading_instructions::LOAD_REGISTER_INTO_HL_LOCATION_PATTERN;
//         use crate::register::Register;

//         fn run_operation(opcode: u8) -> (Emulator, u16) {
//             let h = 0x03u8;
//             let l = 0x03u8;
//             let hl = u16::from_le_bytes([l, h]);

//             let mut memory_state = HashMap::<u16, u8>::new();
//             memory_state.insert(hl, 0x00u8);

//             let mut emulator = test_emulator(memory_state);

//             emulator.set_a(3u8);
//             emulator.set_register(Register::B, 3u8);
//             emulator.set_register(Register::C, 3u8);
//             emulator.set_register(Register::D, 3u8);
//             emulator.set_register(Register::E, 3u8);
//             emulator.set_register(Register::H, h);
//             emulator.set_register(Register::L, l);

//             emulator.process_opcode(opcode);

//             (emulator, hl)
//         }

//         #[test]
//         fn cycles() {
//             for opcode in Opcode::variations(LOAD_REGISTER_INTO_HL_LOCATION_PATTERN) {
//                 let (emulator, _) = run_operation(opcode);

//                 assert_eq!(emulator.cycles(), 2);
//             }
//         }

//         #[test]
//         fn operation() {
//             // Test each variation
//             for opcode in Opcode::variations(LOAD_REGISTER_INTO_HL_LOCATION_PATTERN) {
//                 let (emulator, location) = run_operation(opcode);

//                 let source_register = parse_register(opcode & 0b00000111u8).unwrap();

//                 assert_eq!(
//                     emulator.memory_location(location),
//                     0x03u8,
//                     "Failed for source {:?}",
//                     source_register
//                 );
//             }
//         }

//         #[test]
//         fn variations() {
//             assert_eq!(Opcode::variations(LOAD_REGISTER_INTO_HL_LOCATION_PATTERN).len(), 7);
//         }
//     }

//     mod load_register_into_register {
//         use common::opcode::Opcode;

//         use crate::emulator::Emulator;
//         use crate::instruction::loading_instructions::LOAD_REGISTER_INTO_REGISTER_PATTERN;
//         use crate::instruction::parse_register;
//         use crate::register::Register;

//         fn run_operation(opcode: u8) -> Emulator {
//             let mut emulator = Emulator::new();

//             emulator.set_a(0u8);
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
//             for opcode in Opcode::variations(LOAD_REGISTER_INTO_REGISTER_PATTERN) {
//                 let emulator = run_operation(opcode);

//                 assert_eq!(emulator.cycles(), 1);
//             }
//         }

//         #[test]
//         fn operation() {
//             // Test each variation
//             for opcode in Opcode::variations(LOAD_REGISTER_INTO_REGISTER_PATTERN) {
//                 let emulator = run_operation(opcode);

//                 let source_register = parse_register(opcode & 0b00000111u8).unwrap();

//                 let destination_register = opcode.parse_register(0b00111000u8).unwrap();

//                 assert_eq!(
//                     emulator.register(&source_register),
//                     emulator.register(&destination_register),
//                     "Failed for source {:?} -> {:?}",
//                     source_register,
//                     destination_register
//                 );
//             }
//         }

//         #[test]
//         fn variations() {
//             assert_eq!(Opcode::variations(LOAD_REGISTER_INTO_REGISTER_PATTERN).len(), 49);
//         }
//     }
// }
