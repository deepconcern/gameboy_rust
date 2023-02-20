use std::collections::HashMap;

use super::{MemoryComponent, MemoryError};

const STACK_END_ADDRESS: u16 = 0xfffeu16;
const STACK_START_ADDRESS: u16 = 0xff80u16;

pub struct StackComponent {
    memory_state: HashMap<u16, u8>,
}

impl StackComponent {
    pub fn new() -> Self {
        let mut memory_state = HashMap::new();

        for i in STACK_START_ADDRESS..(STACK_END_ADDRESS + 1) {
            memory_state.insert(i, 0x00u8);
        }

        StackComponent { memory_state }
    }
}

impl MemoryComponent for StackComponent {
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

