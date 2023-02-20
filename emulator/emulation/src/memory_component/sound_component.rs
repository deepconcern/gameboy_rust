use std::collections::HashMap;

use super::{MemoryComponent, MemoryError};

const NR_10_ADDRESS: u16 = 0xff10u16;
const NR_11_ADDRESS: u16 = 0xff11u16;
const NR_12_ADDRESS: u16 = 0xff12u16;
const NR_13_ADDRESS: u16 = 0xff13u16;
const NR_14_ADDRESS: u16 = 0xff14u16;
const NR_21_ADDRESS: u16 = 0xff16u16;
const NR_22_ADDRESS: u16 = 0xff17u16;
const NR_23_ADDRESS: u16 = 0xff18u16;
const NR_24_ADDRESS: u16 = 0xff19u16;
const NR_30_ADDRESS: u16 = 0xff1au16;
const NR_31_ADDRESS: u16 = 0xff1bu16;
const NR_32_ADDRESS: u16 = 0xff1cu16;
const NR_33_ADDRESS: u16 = 0xff1du16;
const NR_34_ADDRESS: u16 = 0xff1eu16;
const NR_41_ADDRESS: u16 = 0xff20u16;
const NR_42_ADDRESS: u16 = 0xff21u16;
const NR_43_ADDRESS: u16 = 0xff22u16;
const NR_44_ADDRESS: u16 = 0xff23u16;
const NR_50_ADDRESS: u16 = 0xff24u16;
const NR_51_ADDRESS: u16 = 0xff25u16;
const NR_52_ADDRESS: u16 = 0xff26u16;
const WAVE_PATTERN_RAM_START_ADDRESS: u16 = 0xff30u16;
const WAVE_PATTERN_RAM_END_ADDRESS: u16 = 0xff3fu16;

pub struct SoundComponent {
    memory_state: HashMap<u16, u8>,
}

impl SoundComponent {
    pub fn new() -> Self {
        let mut memory_state = HashMap::new();

        memory_state.insert(NR_10_ADDRESS, 0x00u8);
        memory_state.insert(NR_11_ADDRESS, 0x00u8);
        memory_state.insert(NR_12_ADDRESS, 0x00u8);
        memory_state.insert(NR_13_ADDRESS, 0x00u8);
        memory_state.insert(NR_14_ADDRESS, 0x00u8);
        memory_state.insert(NR_21_ADDRESS, 0x00u8);
        memory_state.insert(NR_22_ADDRESS, 0x00u8);
        memory_state.insert(NR_23_ADDRESS, 0x00u8);
        memory_state.insert(NR_24_ADDRESS, 0x00u8);
        memory_state.insert(NR_30_ADDRESS, 0x00u8);
        memory_state.insert(NR_31_ADDRESS, 0x00u8);
        memory_state.insert(NR_32_ADDRESS, 0x00u8);
        memory_state.insert(NR_33_ADDRESS, 0x00u8);
        memory_state.insert(NR_34_ADDRESS, 0x00u8);
        memory_state.insert(NR_41_ADDRESS, 0x00u8);
        memory_state.insert(NR_42_ADDRESS, 0x00u8);
        memory_state.insert(NR_43_ADDRESS, 0x00u8);
        memory_state.insert(NR_44_ADDRESS, 0x00u8);
        memory_state.insert(NR_50_ADDRESS, 0x00u8);
        memory_state.insert(NR_51_ADDRESS, 0x00u8);
        memory_state.insert(NR_52_ADDRESS, 0x00u8);

        for i in WAVE_PATTERN_RAM_START_ADDRESS..(WAVE_PATTERN_RAM_END_ADDRESS + 1) {
            memory_state.insert(i, 0x00u8);
        }

        SoundComponent { memory_state }
    }
}

impl MemoryComponent for SoundComponent {
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