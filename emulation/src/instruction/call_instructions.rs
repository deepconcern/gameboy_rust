use crate::{
    emulator::Emulator,
    instruction::{general_instructions::ei, Instruction, OpResult},
    opcode::Opcode,
};

fn store_program_counter(emulator: &mut Emulator) -> OpResult {
    let [low_value, high_value] = emulator.program_counter().to_le_bytes();

    emulator.write(emulator.stack_pointer() - 1, high_value)?;
    emulator.write(emulator.stack_pointer() - 2, low_value)?;

    Ok(())
}

/// CALL nn
/// 
/// (SP - 1) <- PCH
/// (SP - 2) <- PCL
/// PC <- nn
/// SP <- SP - 2
/// 
/// Pushes the pc high and low bytes to memory specified by sp, loads 16-bit
/// immediate operand nn into pc, and decrements the sp by two.
pub fn call(emulator: &mut Emulator, _: u8) -> OpResult {
    store_program_counter(emulator)?;

    let nn = emulator.read_immediate_nn()?;

    emulator.set_program_counter(nn);

    emulator.set_stack_pointer(emulator.stack_pointer() - 2);

    Ok(())
}

pub const CALL: Instruction = Instruction {
    name: "CALL nn",
    op: call,
    pattern: "11 001 101",
    requires_prefix: false,
};

/// CALL cc, nn
/// 
/// if cc,
/// (SP - 1) <- PCH
/// (SP - 2) <- PCL
/// PC <- nn
/// SP <- SP - 2
/// 
/// Pushes the pc high and low bytes to memory specified by sp, loads 16-bit
/// immediate operand nn into pc, and decrements the sp by two if cc is true.
pub fn call_if_condition(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let condition = opcode.parse_condition(0b00_011_000)?;
    
    if condition.check(emulator) {
        call(emulator, opcode)?;
    }

    Ok(())
}

pub const CALL_IF_CONDITION: Instruction = Instruction {
    name: "CALL cc, nn",
    op: call_if_condition,
    pattern: "11 0cc 100",
    requires_prefix: false,
};

/// RET
/// 
/// PCL <- (SP)
/// PCH <- (SP + 1)
/// SP <- SP + 2
/// 
/// Loads into PC memory specified by sp, and increments sp by two.
pub fn ret(emulator: &mut Emulator, _: u8) -> OpResult {
    let low_value = emulator.read(emulator.stack_pointer())?;
    let high_value = emulator.read(emulator.stack_pointer() + 1)?;

    emulator.set_program_counter(u16::from_le_bytes([low_value, high_value]));

    emulator.set_stack_pointer(emulator.stack_pointer() + 2);

    Ok(())
}

pub const RET: Instruction = Instruction {
    name: "RET",
    op: ret,
    pattern: "11 001 001",
    requires_prefix: false,
};

/// RET cc
/// 
/// if cc,
/// PCL <- (SP)
/// PCH <- (SP + 1)
/// SP <- SP + 2
/// 
/// Loads into PC memory specified by sp, and increments sp by two if cc is
/// true.
pub fn ret_if_condition(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let condition = opcode.parse_condition(0b_00_011_000)?;
    
    if condition.check(emulator) {
        ret(emulator, opcode)?;
    };

    Ok(())
}

pub const RET_IF_CONDITION: Instruction = Instruction {
    name: "RET cc",
    op: ret_if_condition,
    pattern: "11 0cc 000",
    requires_prefix: false,
};

/// RETI
/// 
/// PCL <- (SP)
/// PCH <- (SP + 1)
/// SP <- SP + 2
/// 
/// Enables ime, and oads into PC memory specified by sp, and increments sp by
/// two.
pub fn reti(emulator: &mut Emulator, opcode: u8) -> OpResult {
    ei(emulator, opcode)?;
    ret(emulator, opcode)?;

    Ok(())
}

pub const RETI: Instruction = Instruction {
    name: "RETI",
    op: reti,
    pattern: "11 011 001",
    requires_prefix: false,
};

/// RST t
/// 
/// (SP - 1) <- PCH
/// (SP - 2) <- PCL
/// SP <- SP - 2
/// PCH <- 0
/// PCL <- P
/// 
/// Pushes the pc high and low bytes to memory specified by sp, decrements the
/// sp by two, and sets pc to page 0 address specified by operand t.
pub fn reset_to_page(emulator: &mut Emulator, opcode: u8) -> OpResult {
    let page = opcode.parse_page(0b00_111_000)?;

    store_program_counter(emulator)?;

    emulator.set_stack_pointer(emulator.stack_pointer() - 2);

    emulator.set_program_counter(page);

    Ok(())
}

pub const RESET_TO_PAGE: Instruction = Instruction {
    name: "RST t",
    op: reset_to_page,
    pattern: "11 ttt 111",
    requires_prefix: false,
};

pub fn add_call_instructions(emulator: &mut Emulator) {
    emulator.add_instruction(CALL);
    emulator.add_instruction(CALL_IF_CONDITION);
    emulator.add_instruction(RESET_TO_PAGE);
    emulator.add_instruction(RET);
    emulator.add_instruction(RET_IF_CONDITION);
    emulator.add_instruction(RETI);
}