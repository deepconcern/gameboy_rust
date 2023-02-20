use std::collections::HashMap;


use crate::bits::{bit_add, BitAddResult};
use crate::flag::Flag;
use crate::memory_component::MemoryError;
use crate::memory_mapping::MemoryMapping;
use crate::register::{Register, RegisterPair};

pub struct ProcessorState {
    pub memory_mapping: MemoryMapping,
    pub program_counter: u16,
    pub registers: HashMap<Register, u8>,
}

impl ProcessorState {
    pub fn new() -> ProcessorState {
        ProcessorState {
            memory_mapping: MemoryMapping::new(),
            program_counter: 0u16,
            registers: HashMap::from([
                (Register::A, 0u8),
                (Register::B, 1u8),
                (Register::C, 0u8),
                (Register::D, 0u8),
                (Register::E, 0u8),
                (Register::F, 0u8),
                (Register::H, 0u8),
                (Register::L, 0u8),
            ]),
        }
    }

    pub fn add(&mut self, a: u8, b: u8, with_carry: bool) -> u8 {
        let has_carry = if with_carry {
            self.flag_enabled(Flag::CY)
        } else {
            false
        };

        let BitAddResult {
            carry,
            half_carry,
            value,
        } = bit_add(a, b, has_carry);

        if carry {
            self.set_flag(Flag::CY)
        } else {
            self.reset_flag(Flag::CY)
        };
        if half_carry {
            self.set_flag(Flag::H)
        } else {
            self.reset_flag(Flag::H)
        };

        // Addition ALWAYS sets N to 0
        self.reset_flag(Flag::N);

        value
    }

    pub fn add_to_a(&mut self, value: u8, with_carry: bool) -> u8 {
        self.add(self.registers[&Register::A], value, with_carry)
    }

    pub fn flag_enabled(&self, flag: Flag) -> bool {
        (self.registers[&Register::F] & (flag as u8)) > 0
    }

    pub fn flip_flag(&mut self, flag: Flag) {
        self.registers
            .insert(Register::F, self.registers[&Register::F] ^ (flag as u8));
    }

    pub fn get_immediate_e(&mut self) -> Result<i8, MemoryError> {
        let value = self.get_immediate_n()?;

        Ok(value as i8)
    }

    pub fn get_immediate_n(&mut self) -> Result<u8, MemoryError> {
        let value = self.read(self.program_counter);

        self.program_counter = self.program_counter.wrapping_add(1u16);

        value
    }

    pub fn get_immediate_nn(&mut self) -> Result<u16, MemoryError> {
        let low = self.get_immediate_n()?;

        let high = self.get_immediate_n()?;

        Ok(u16::from_le_bytes([low, high]))
    }

    pub fn get_register_pair(&self, register_pair: &RegisterPair) -> u16 {
        let low = match register_pair {
            RegisterPair::Af => self.registers[&Register::F],
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

    pub fn read(&self, location: u16) -> Result<u8, MemoryError> {
        self.memory_mapping.read(location)
    }

    pub fn reset_flag(&mut self, flag: Flag) {
        self.registers
            .insert(Register::F, self.registers[&Register::F] & !(flag as u8));
    }

    pub fn set_flag(&mut self, flag: Flag) {
        self.registers
            .insert(Register::F, self.registers[&Register::F] | (flag as u8));
    }

    pub fn write(&mut self, location: u16, value: u8) -> Result<(), MemoryError> {
        self.memory_mapping.write(location, value)
    }
}

impl Default for ProcessorState {
    fn default() -> Self {
        ProcessorState::new()
    }
}
