use std::collections::HashMap;

use emulation::{Emulator, MemoryComponent, MemoryError, addresses::PROGRAM_COUNTER_START, register::Register, instruction::general_instructions::PREFIX, opcode::OpcodePattern};

pub struct TestComponent {
    memory_state: HashMap<u16, u8>,
}

impl TestComponent {
    pub fn new(memory_state: HashMap<u16, u8>) -> Self {
        TestComponent { memory_state }
    }
}

impl MemoryComponent for TestComponent {
    fn mapped_locations(&self) -> Vec<u16> {
        self.memory_state.keys().cloned().collect()
    }

    fn read(&self, location: u16) -> Result<u8, MemoryError> {
        self.memory_state.get(&location).copied().ok_or(MemoryError::ReadError(location, "invalid state"))
    }

    fn write(&mut self, location: u16, value: u8) -> Result<(), MemoryError> {
        if self.memory_state.contains_key(&location) {
            self.memory_state.insert(location, value);

            Ok(())
        } else {
            Err(MemoryError::WriteError(location, value, "invalid state"))
        }
    }
}

pub fn build_memory(opcode: u8, prefixed: bool) -> HashMap<u16, u8> {
    let mut memory_state = HashMap::new();

    if prefixed {
        let prefix_opcode = PREFIX.pattern.opcodes()[0];

        memory_state.insert(PROGRAM_COUNTER_START, prefix_opcode);
        memory_state.insert(PROGRAM_COUNTER_START + 1, opcode);
    } else {
        memory_state.insert(PROGRAM_COUNTER_START, opcode);
    };

    memory_state
}

pub fn complex_emulator(memory_state: HashMap<u16, u8>) -> Emulator {
    let mut emulator = Emulator::new();

    emulation::add_instructions(&mut emulator);

    emulator.add_memory_component(Box::new(TestComponent::new(memory_state)));

    emulator
}

#[allow(dead_code)]
pub fn simple_emulator(opcode: u8) -> Emulator {
    let memory_state = build_memory(opcode, false);

    complex_emulator(memory_state)
}

#[allow(dead_code)]
pub fn prefixed_emulator(opcode: u8) -> Emulator {
    let memory_state = build_memory(opcode, true);

    complex_emulator(memory_state)
}

#[allow(dead_code)]
pub fn setup_read_immediate_e(opcode: u8, value: i8, prefixed: bool) -> Emulator {
    setup_read_immediate_n(opcode, value as u8, prefixed)
}

#[allow(dead_code)]
pub fn setup_read_immediate_n(opcode: u8, value: u8, prefixed: bool) -> Emulator {
    let mut memory_state = build_memory(opcode, prefixed);

    let offset = memory_state.len() as u16;

    memory_state.insert(PROGRAM_COUNTER_START + offset, value);

    complex_emulator(memory_state)
}

#[allow(dead_code)]
pub fn setup_read_immediate_nn(opcode: u8, value: u16, prefixed: bool) -> Emulator {
    let [low_value, high_value] = value.to_le_bytes();

    let mut memory_state = build_memory(opcode, prefixed);

    let offset = memory_state.len() as u16;

    memory_state.insert(PROGRAM_COUNTER_START + offset, low_value);
    memory_state.insert(PROGRAM_COUNTER_START + offset + 1, high_value);

    complex_emulator(memory_state)
}

#[allow(dead_code)]
pub fn setup_read_hl_location(opcode: u8, value: u8, prefixed: bool) -> Emulator {
    let h = 0x03u8;
    let l = 0x03u8;

    let hl = u16::from_le_bytes([l, h]);

    let mut memory_state = build_memory(opcode, prefixed);

    memory_state.insert(hl, value);

    let mut emulator = complex_emulator(memory_state);

    emulator.set_register(Register::H, h);
    emulator.set_register(Register::L, l);

    emulator
}