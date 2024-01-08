use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum MemoryError {
    ReadError(u16, &'static str),
    WriteError(u16, u8, &'static str),
}

impl Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryError::ReadError(location, message) => {
                write!(f, "Read error ({:#06x}): {}", location, message)
            },
            MemoryError::WriteError(location, value, message) => {
                write!(f, "Write error ({:#06x} <- {:#04x}): {}", location, value, message)
            },
        }
    }
}

pub trait MemoryComponent {
    fn mapped_locations(&self) -> Vec<u16> {
        (0u16..u16::MAX).collect()
    }

    fn read(&self, location: u16) -> Result<u8, MemoryError> {
        Err(MemoryError::ReadError(location, "unimplemented"))
    }

    fn write(&mut self, location: u16, value: u8) -> Result<(), MemoryError> {
        Err(MemoryError::WriteError(location, value, "unimplemented"))
    }
}
