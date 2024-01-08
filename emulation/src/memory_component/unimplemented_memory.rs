use super::MemoryComponent;

pub struct UnimplementedMemory {}

impl UnimplementedMemory {
    pub fn new() -> Self {
        UnimplementedMemory {}
    }
}

impl MemoryComponent for UnimplementedMemory {}

#[cfg(test)]
mod tests {
    use super::super::{MemoryComponent, MemoryError};

    use super::UnimplementedMemory;

    #[test]
    fn read() {
        let unimplemented_memory = UnimplementedMemory::new();

        let location = 0x0000u16;

        match unimplemented_memory.read(location) {
            Err(MemoryError::ReadError(actual_location, actual_message)) => {
                assert_eq!(actual_location, location);
                assert_eq!(actual_message, "unimplemented");
            },
            _ => panic!("invalid state"),
        };
    }

    #[test]
    fn write() {
        let mut unimplemented_memory = UnimplementedMemory::new();

        let location = 0x0000u16;
        let value = 0x01u8;

        match unimplemented_memory.write(location, value) {
            Err(MemoryError::WriteError(actual_location, actual_value, actual_message)) => {
                assert_eq!(actual_location, location);
                assert_eq!(actual_value, value);
                assert_eq!(actual_message, "unimplemented");
            },
            _ => panic!("invalid state"),
        };
    }
}