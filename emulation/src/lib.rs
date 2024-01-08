extern crate num;
#[macro_use]
extern crate num_derive;
extern crate num_traits;

pub mod addresses;
mod bits;
mod condition;
mod emulator;
pub mod flag;
pub mod instruction;
mod memory_component;
mod memory_mapping;
pub mod opcode;
pub mod register;

pub use crate::{
    emulator::Emulator,
    memory_component::{MemoryComponent, MemoryError},
    register::Register,
};
use instruction::{
    arithmetic_instructions::add_arithmetic_instructions,
    bit_instructions::add_bit_instructions,
    call_instructions::add_call_instructions,
    general_instructions::add_general_instructions,
    jump_instructions::add_jump_instructions,
    loading_instructions::add_loading_instructions,
    logical_instructions::add_logical_instructions,
    rotating_instructions::add_rotating_instructions,
};
use memory_component::{SerialTransferComponent, SoundComponent, StackComponent, UnusableRamComponent, WorkRamComponent};

pub fn add_instructions(emulator: &mut Emulator) {
    add_arithmetic_instructions(emulator);
    add_bit_instructions(emulator);
    add_call_instructions(emulator);
    add_general_instructions(emulator);
    add_jump_instructions(emulator);
    add_loading_instructions(emulator);
    add_logical_instructions(emulator);
    add_rotating_instructions(emulator);
}


impl Default for Emulator {
    fn default() -> Self {
        let mut emulator = Emulator::new();

        // Add components
        emulator.add_memory_component(Box::new(SerialTransferComponent::new()));
        emulator.add_memory_component(Box::new(SoundComponent::new()));
        emulator.add_memory_component(Box::new(StackComponent::new()));
        emulator.add_memory_component(Box::new(UnusableRamComponent::new()));
        emulator.add_memory_component(Box::new(WorkRamComponent::new()));

        // Add instructions
        add_instructions(&mut emulator);

        emulator
    }
}