use std::collections::HashMap;

use super::{MemoryComponent, MemoryError};

const WORK_RAM_START_ADDRESS: u16 = 0xc000u16;
const WORK_RAM_END_ADDRESS: u16 = 0xdfffu16;

const ECHO_RAM_START_ADDRESS: u16 = 0xe000u16;
const ECHO_RAM_END_ADDRESS: u16 = 0xfdffu16;

pub struct WorkRamComponent {
    memory_state: HashMap<u16, u8>,
}

impl WorkRamComponent {
    pub fn new() -> Self {
        let mut memory_state = HashMap::new();

        for i in ECHO_RAM_START_ADDRESS..(ECHO_RAM_END_ADDRESS + 1) {
            memory_state.insert(i, 0x00u8);
        }

        for i in WORK_RAM_START_ADDRESS..(WORK_RAM_END_ADDRESS + 1) {
            memory_state.insert(i, 0x00u8);
        }

        WorkRamComponent { memory_state }
    }
}

impl MemoryComponent for WorkRamComponent {
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

