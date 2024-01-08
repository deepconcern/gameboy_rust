use crate::emulator::{Emulator, EmulationState};
use crate::flag::Flag;
use crate::instruction::{Instruction, OpResult};

/// CPL A
/// 
/// A <- ^A
/// 
/// Takes the one's complement of register A and stores the result in register
/// A.
pub fn complement_a(emulator: &mut Emulator, _: u8) -> OpResult {
    emulator.set_a(!emulator.a());

    Ok(())
}

pub const COMPLEMENT_A: Instruction = Instruction {
    name: "CPL A",
    op: complement_a,
    pattern: "00 101 111",
    requires_prefix: false,
};

/// DAA
/// 
/// Decimal adjust accu
/// 
/// Adjusts a after addition and subtraction.
pub fn decimal_adjust_a(emulator: &mut Emulator, _: u8) -> OpResult {
    let mut a = emulator.a();

    let h = emulator.flag(Flag::H);
    let n = emulator.flag(Flag::N);
    let cy = emulator.flag(Flag::CY);

    let mut carry = false;
    let mut offset = 0u8;

    if (!n && a & 0x0fu8 > 0x09) || h {
        offset |= 0x06u8;
    };

    if (!n && a & 0xf0u8 > 0x90) || cy {
        carry = true;
        offset |= 0x60u8;
    };

    if n {
        a = a.wrapping_sub(offset);
    } else {
        a = a.wrapping_add(offset);
    };

    emulator.set_a(a);

    emulator.set_flag(Flag::CY, carry);
    emulator.set_flag(Flag::H, false);
    emulator.set_flag(Flag::Z, a == 0);

    Ok(())
}

pub const DECIMAL_ADJUST_A: Instruction = Instruction {
    name: "DAA",
    op: decimal_adjust_a,
    pattern: "00 100 111",
    requires_prefix: false,
};

/// DI
/// 
/// IME <- 0
/// 
/// Sets the ime to 0.
pub fn di(emulator: &mut Emulator, _: u8) -> OpResult {
    emulator.set_interrupt_master_enable(false);

    Ok(())
}

pub const DI: Instruction = Instruction {
    name: "DI",
    op: di,
    pattern: "11 110 011",
    requires_prefix: false,
};

/// EI
/// 
/// IME <- 1
/// 
/// Sets the ime to 1.
pub fn ei(emulator: &mut Emulator, _: u8) -> OpResult {
    emulator.set_interrupt_master_enable(true);

    Ok(())
}

pub const EI: Instruction = Instruction {
    name: "EI",
    op: ei,
    pattern: "11 111 011",
    requires_prefix: false,
};

/// CCF
/// 
/// CY <- ^CY
/// 
/// Flips CY.
pub fn flip_carry(emulator: &mut Emulator, _: u8) -> OpResult {
    emulator.set_flag(Flag::CY, !emulator.flag(Flag::CY));

    Ok(())
}

pub const FLIP_CARRY: Instruction = Instruction {
    name: "CCF",
    op: flip_carry,
    pattern: "00 111 111",
    requires_prefix: false,
};

/// HALT
/// 
/// Halt
/// 
/// Sets the emulator to HALT mode.
pub fn halt(emulator: &mut Emulator, _: u8) -> OpResult {
    emulator.set_state(EmulationState::Halt);

    Ok(())
}

pub const HALT: Instruction = Instruction {
    name: "HALT",
    op: halt,
    pattern: "01 110 110",
    requires_prefix: false,
};

/// NOP
/// 
/// No operation
/// 
/// Performs no operation.
pub fn noop(_: &mut Emulator, _: u8) -> OpResult {
    Ok(())
}

pub const NOOP: Instruction = Instruction {
    name: "NOP",
    op: noop,
    pattern: "00 000 000",
    requires_prefix: false,
};

pub fn prefix(emulator: &mut Emulator, _: u8) -> OpResult {
    emulator.set_prefix(true);

    Ok(())
}

pub const PREFIX: Instruction = Instruction {
    name: "PREFIX",
    op: prefix,
    pattern: "11 001 011",
    requires_prefix: false,
};

/// SCY
/// 
/// CY <- 1
/// 
/// Sets the CY to 1.
pub fn set_carry(emulator: &mut Emulator, _: u8) -> OpResult {
    emulator.set_flag(Flag::CY, true);

    Ok(())
}

pub const SET_CARRY: Instruction = Instruction {
    name: "SCY",
    op: set_carry,
    pattern: "00 110 111",
    requires_prefix: false,
};

/// STOP
/// 
/// Stop
/// 
/// Sets the emulator to STOP mode.
pub fn stop(emulator: &mut Emulator, _: u8) -> OpResult {
    emulator.set_state(EmulationState::Stop);

    Ok(())
}

pub const STOP: Instruction = Instruction {
    name: "STOP",
    op: stop,
    pattern: "00 010 000",
    requires_prefix: false,
};

// pub fn unimplemented(_: &mut Emulator, opcode: u8) -> OpResult {
//     Err(OpError::Unimplemented(false, opcode))
// }

// pub const UNIMPLEMENTED : Instruction = Instruction {
//     name: "-",
//     op: unimplemented,
//     pattern: "",
//     requires_prefix: false,
// };

pub const UNIMPLEMENTED_OPCODES: [u8; 11] = [
    0xd3, // 0b11_010_011,
    0xdb, // 0b11_011_011,
    0xdd, // 0b11_011_101,
    0xe3, // 0b11_100_011,
    0xe4, // 0b11_100_100,
    0xeb, // 0b11_101_011,
    0xec, // 0b11_101_100,
    0xed, // 0b11_101_101,
    0xf4, // 0b11_110_100,
    0xfc, // 0b11_111_100,
    0xfd, // 0b11_111_101,
];

pub fn add_general_instructions(emulator: &mut Emulator) {
    emulator.add_instruction(COMPLEMENT_A);
    emulator.add_instruction(DECIMAL_ADJUST_A);
    emulator.add_instruction(DI);
    emulator.add_instruction(EI);
    emulator.add_instruction(FLIP_CARRY);
    emulator.add_instruction(HALT);
    emulator.add_instruction(NOOP);
    emulator.add_instruction(PREFIX);
    emulator.add_instruction(SET_CARRY);
    emulator.add_instruction(STOP);
}