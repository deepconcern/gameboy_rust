use super::MemoryComponent;

const SB_ADDRESS: u16 = 0xff01u16;
const SC_ADDRESS: u16 = 0xff02u16;

pub struct SerialTransferComponent {}

impl SerialTransferComponent {
    pub fn new() -> Self {
        SerialTransferComponent {  }
    }
}

impl MemoryComponent for SerialTransferComponent {
    fn mapped_locations(&self) -> Vec<u16> {
        vec![SB_ADDRESS, SC_ADDRESS]
    }
}