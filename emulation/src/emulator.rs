use std::collections::HashMap;

use crate::addresses::PROGRAM_COUNTER_START;
use crate::bits::{bit_add, bit_subtract, SignedInt, UnsignedInt};
use crate::flag::Flag;
use crate::instruction::{OpError, OpResult};
use crate::instruction::{Instruction, Op};
use crate::memory_component::MemoryError;
use crate::memory_mapping::MemoryMapping;
use crate::opcode::OpcodePattern;
use crate::register::{Register, RegisterPair};
use crate::memory_component::MemoryComponent;

pub enum EmulationState {
    Halt,
    Run,
    Stop,
}

pub struct Emulator {
    cycles_processed: usize,
    flags: u8,
    instructions: Vec<Op>,
    instruction_map: HashMap<(bool, u8), usize>,
    interrupt_master_enable: bool,
    jumped: bool,
    memory_mapping: MemoryMapping,
    name_map: HashMap<(bool, u8), String>,
    prefixed: bool,
    program_counter: u16,
    registers: HashMap<Register, u8>,
    stack_pointer: u16,
    state: EmulationState,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            cycles_processed: 0usize,
            flags: 0x00u8,
            interrupt_master_enable: false,
            instructions: Vec::new(),
            instruction_map: HashMap::new(),
            jumped: false,
            memory_mapping: MemoryMapping::new(),
            name_map: HashMap::new(),
            prefixed: false,
            program_counter: PROGRAM_COUNTER_START,
            registers: HashMap::from([
                (Register::A, 0u8),
                (Register::B, 1u8),
                (Register::C, 0u8),
                (Register::D, 0u8),
                (Register::E, 0u8),
                // (Register::F, 0u8),
                (Register::H, 0u8),
                (Register::L, 0u8),
            ]),
            stack_pointer: 0u16,
            state: EmulationState::Run,
        }
    }

    pub fn a(&self) -> u8 {
        self.register(&Register::A)
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        let instruction_index = self.instructions.len();

        self.instructions.push(instruction.op);

        for opcode in instruction.pattern.opcodes() {
            if self.name_map.contains_key(&(instruction.requires_prefix, opcode)) {
                panic!("Failed to insert opcode {:#04x} for '{}'. Opcode has already been implemented for '{}'", opcode, instruction.name, self.name_map.get(&(instruction.requires_prefix, opcode)).unwrap());
            }

            self.instruction_map.insert((instruction.requires_prefix, opcode), instruction_index);
            self.name_map.insert((instruction.requires_prefix, opcode), String::from(instruction.name));
        }
    }

    pub fn add_memory_component(&mut self, memory_component: Box<dyn MemoryComponent>) {
        self.memory_mapping.register_component(memory_component);
    }

    pub fn add_signed<S: SignedInt, U: TryFrom<S> + UnsignedInt>(&mut self, a: U, b: S, with_carry: bool) -> U {
        if b.is_negative() {
            let b_unsigned: U = b.abs().try_into().ok().unwrap();
            let value = self.subtract_unsigned(a, b_unsigned, with_carry);

            self.set_flag(Flag::N, false);

            value
        } else {
            let b_unsigned: U = b.try_into().ok().unwrap();
            self.add_unsigned(a, b_unsigned, with_carry)
        }
    }

    pub fn add_to_a(&mut self, value: u8, with_carry: bool) {
        let value = self.add_unsigned(self.registers[&Register::A], value, with_carry);

        self.registers.insert(Register::A, value);
    }

    pub fn add_unsigned<U: UnsignedInt>(&mut self, a: U, b: U, with_carry: bool) -> U {
        let has_carry = with_carry && self.flag(Flag::CY);

        let (value, carry, half_carry) = bit_add(a, b, has_carry);

        self.set_flag(Flag::CY, carry);
        self.set_flag(Flag::H, half_carry);

        // Addition ALWAYS sets N to 0
        self.set_flag(Flag::N, false);

        self.set_flag(Flag::Z, value.is_zero());

        value
    }

    pub fn bitwise_and_with_a(&mut self, value: u8) {
        let value = self.register(&Register::A) & value;

        self.set_flag(Flag::CY, false);
        self.set_flag(Flag::H, true);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::Z, value == 0u8);

        self.set_a(value);
    }

    pub fn bitwise_or_with_a(&mut self, value: u8) {
        let value = self.register(&Register::A) | value;

        self.set_flag(Flag::CY, false);
        self.set_flag(Flag::H, false);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::Z, value == 0u8);

        self.set_a(value);
    }

    pub fn bitwise_xor_with_a(&mut self, value: u8) {
        let value = self.register(&Register::A) ^ value;

        self.set_flag(Flag::CY, false);
        self.set_flag(Flag::H, false);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::Z, value == 0u8);

        self.set_a(value);
    }

    pub fn cycles(&self) -> usize {
        self.cycles_processed
    }

    pub fn flag(&self, flag: Flag) -> bool {
        self.flags & (flag as u8) > 0
    }

    pub fn flip_flag(&mut self, flag: Flag) {
        self.flags = self.flags ^ (flag as u8);
    }

    pub fn jump_relative_to(&mut self, value: i8) {
        let location = if value.is_negative() {
            self.program_counter.wrapping_sub(value.abs() as u16)
        } else {
            self.program_counter.wrapping_add(value as u16)
        };

        self.jump_to(location);
    }

    pub fn jump_to(&mut self, location: u16) {
        self.program_counter = location;
        self.jumped = true;
    }

    pub fn hl(&self) -> u16 {
        self.register_pair(&RegisterPair::Hl)
    }

    pub fn instruction_name(&self, op: (bool, u8)) -> Option<&String> {
        self.name_map.get(&op)
    }

    pub fn interrupt_master_enable(&self) -> bool {
        self.interrupt_master_enable
    }

    pub fn memory_location(&self, location: u16) -> u8 {
        self.memory_mapping.read(location).unwrap()
    }

    pub fn prefixed(&self) -> bool {
        self.prefixed
    }

    pub fn process_cycles(&mut self) {
        self.cycles_processed = 0;
    }

    pub fn process_opcode(&mut self) -> OpResult {
        let opcode = self.read(self.program_counter)?;

        self.program_counter = self.program_counter.wrapping_add(1);

        let op_index = if self.prefixed {
            self.prefixed = false;

            self.instruction_map.get(&(true, opcode)).ok_or(OpError::Unimplemented(false, opcode))?
        } else {
            self.instruction_map.get(&(false, opcode)).ok_or(OpError::Unimplemented(true, opcode))?
        };

        let op = &self.instructions[*op_index];
        
        op(self, opcode)?;

        Ok(())
    }

    pub fn program_counter(&self) -> u16 {
        self.program_counter
    }

    pub fn read(&mut self, location: u16) -> Result<u8, MemoryError> {
        // Process cycle
        self.cycles_processed += 1;

        self.memory_mapping.read(location)
    }

    pub fn read_hl_location(&mut self) -> Result<u8, MemoryError> {
        let location = self.register_pair(&RegisterPair::Hl);

        self.read(location)
    }

    pub fn read_immediate_e(&mut self) -> Result<i8, MemoryError> {
        let value = self.read_immediate_n()?;

        Ok(value as i8)
    }

    pub fn read_immediate_n(&mut self) -> Result<u8, MemoryError> {
        let value = self.read(self.program_counter);

        self.program_counter = self.program_counter.wrapping_add(1u16);

        value
    }

    pub fn read_immediate_nn(&mut self) -> Result<u16, MemoryError> {
        let low = self.read_immediate_n()?;

        let high = self.read_immediate_n()?;

        Ok(u16::from_le_bytes([low, high]))
    }

    pub fn register(&self, register: &Register) -> u8 {
        self.registers[register]
    }

    pub fn register_pair(&self, register_pair: &RegisterPair) -> u16 {
        let low = match register_pair {
            RegisterPair::Af => self.flags,
            RegisterPair::Bc => self.registers[&Register::C],
            RegisterPair::De => self.registers[&Register::E],
            RegisterPair::Hl => self.registers[&Register::L],
        };

        let high = match register_pair {
            RegisterPair::Af => self.registers[&Register::A],
            RegisterPair::Bc => self.registers[&Register::B],
            RegisterPair::De => self.registers[&Register::D],
            RegisterPair::Hl => self.registers[&Register::H],
        };

        u16::from_le_bytes([low, high])
    }

    pub fn set_a(&mut self, value: u8) {
        self.set_register(Register::A, value);
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        self.flags = if value {
            self.flags | (flag as u8)
        } else {
            self.flags & !(flag as u8)
        };
    }

    pub fn set_hl(&mut self, value: u16) {
        self.set_register_pair(RegisterPair::Hl, value);
    }

    pub fn set_interrupt_master_enable(&mut self, value: bool) {
        self.interrupt_master_enable = value;
    }

    pub fn set_prefix(&mut self, value: bool) {
        self.prefixed = value;
    }
    
    pub fn set_program_counter(&mut self, value: u16) {
        self.program_counter = value;

        self.cycles_processed += 1;
    }

    pub fn set_register(&mut self, register: Register, value: u8) {
        self.registers.insert(register, value);
    }

    pub fn set_register_pair(&mut self, register_pair: RegisterPair, value: u16) {
        let [low_value, high_value] = value.to_le_bytes();

        if register_pair == RegisterPair::Af {
            self.set_register(Register::A, low_value);
            self.flags = high_value;
        } else {
            let (low_register, high_register) = register_pair.to_registers();
    
            self.set_register(low_register, low_value);
            self.set_register(high_register, high_value);
        }
    }

    pub fn set_stack_pointer(&mut self, value: u16) {
        self.stack_pointer = value;
    }

    pub fn set_state(&mut self, value: EmulationState) {
        self.state = value;
    }

    pub fn stack_pointer(&self) -> u16 {
        self.stack_pointer
    }

    pub fn subtract_from_a(&mut self, value: u8, with_carry: bool) {
        let value = self.subtract_unsigned(self.register(&Register::A), value, with_carry);

        self.set_register(Register::A, value);
    }

    pub fn subtract_unsigned<U: UnsignedInt>(&mut self, a: U, b: U, with_carry: bool) -> U {
        let has_carry = with_carry && self.flag(Flag::CY);

        let (dif, borrow, half_borrow) = bit_subtract(a, b, has_carry);

        self.set_flag(Flag::CY, borrow);
        self.set_flag(Flag::H, half_borrow);

        // Subtraction ALWAYS sets N to 1
        self.set_flag(Flag::N, true);

        self.set_flag(Flag::Z, dif.is_zero());

        dif
    }

    pub fn write(&mut self, location: u16, value: u8) -> Result<(), MemoryError> {
        // Process cycle
        self.cycles_processed += 1;

        self.memory_mapping.write(location, value)
    }

    pub fn write_hl_location(&mut self, value: u8) -> Result<(), MemoryError> {
        let location = self.register_pair(&RegisterPair::Hl);

        self.write(location, value)
    }
}