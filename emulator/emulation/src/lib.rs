
extern crate common;
#[macro_use]
extern crate macros;
extern crate num;
#[macro_use]
extern crate num_derive;
extern crate num_traits;

mod bits;
mod flag;
mod instruction;
mod memory_component;
mod memory_mapping;
mod processor_state;
mod register;

use std::collections::HashMap;

use crate::instruction::{arithmetic_instructions::add_register_to_a, Instruction};
use crate::memory_component::{MemoryComponent, SerialTransferComponent, SoundComponent, StackComponent, UnusableRamComponent, WorkRamComponent};
use crate::processor_state::ProcessorState;
pub use crate::register::Register;

pub struct Emulator {
    instructions: Vec<Box<dyn Instruction>>,
    instruction_map: HashMap<u8, usize>,
    state: ProcessorState,
}

impl Emulator {
    pub fn new() -> Self {
        let mut processor = Emulator {
            instructions: Vec::new(),
            instruction_map: HashMap::new(),
            state: ProcessorState::new(),
        };

        processor.register_memory_component(Box::new(SerialTransferComponent::new()));
        processor.register_memory_component(Box::new(SoundComponent::new()));
        processor.register_memory_component(Box::new(StackComponent::new()));
        processor.register_memory_component(Box::new(UnusableRamComponent::new()));
        processor.register_memory_component(Box::new(WorkRamComponent::new()));

        processor.register_instruction(Box::new(build_instruction!(add_register_to_a)));

        processor
    }

    pub fn get_register(&self, register: &Register) -> u8 {
        self.state.registers[register]
    }

    pub fn register_memory_component(&mut self, memory_component: Box<dyn MemoryComponent>) -> &mut Self {
        self.state.memory_mapping.register_component(memory_component);

        self
    }

    pub fn register_instruction(&mut self, instruction: Box<dyn Instruction>) -> &mut Self {
        let instruction_index = self.instructions.len();

        self.instructions.push(instruction);

        for opcode in self.instructions[instruction_index].variations() {
            self.instruction_map.insert(opcode, instruction_index);
        }

        self
    }

    pub fn process_opcode(&mut self, opcode: u8) -> &mut Self {
        let instruction_index = self.instruction_map.get(&opcode).unwrap();

        let instruction = &self.instructions[*instruction_index];

        instruction.operation(&mut self.state, opcode).ok();

        self
    }
}

impl Default for Emulator {
    fn default() -> Self {
        Emulator::new()
    }
}