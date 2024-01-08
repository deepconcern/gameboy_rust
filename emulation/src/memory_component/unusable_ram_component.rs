use std::collections::HashMap;

use super::{MemoryComponent, MemoryError};

const UNUSABLE_AREA_START_ADDRESS: u16 = 0xc000u16;
const UNUSABLE_AREA_END_ADDRESS: u16 = 0xdfffu16;

pub struct UnusableRamComponent {
    memory_state: HashMap<u16, u8>,
}

impl UnusableRamComponent {
    pub fn new() -> Self {
        let mut memory_state = HashMap::new();

        for i in UNUSABLE_AREA_START_ADDRESS..(UNUSABLE_AREA_END_ADDRESS + 1) {
            memory_state.insert(i, 0x00u8);
        }

        UnusableRamComponent { memory_state }
    }
}

impl MemoryComponent for UnusableRamComponent {
    fn mapped_locations(&self) -> Vec<u16> {
        self.memory_state.keys().cloned().collect()
    }

    fn read(&self, _: u16) -> Result<u8, MemoryError> {
        Ok(0x00u8)
    }

    fn write(&mut self, _: u16, _: u8) -> Result<(), MemoryError> {
        Ok(())
    }
}

