/// An enumeration containing the various registers within the CPU.
#[derive(Debug, Eq, FromPrimitive, Hash, PartialEq)]
#[repr(u8)]
pub enum Register {
    /// The 8-bit A register
    ///
    /// Also known as the accumulator register, this is used as the output for
    /// various arithmetic and logical operations.
    A = 0b111u8,

    /// The 8-bit B register
    B = 0b000u8,

    /// The 8-bit C register
    C = 0b001u8,

    /// The 8-bit D register
    D = 0b010u8,

    /// The 8-bit E register
    E = 0b011u8,

    /// The 8-bit F register
    ///
    /// Also known as the flag register, this is used for storing flag
    /// information.
    // F = 0b110u8,

    /// The 8-bit H register
    H = 0b100u8,

    /// The 8-bit L register
    L = 0b101u8,
}

/// An enumeration of all 16-bit registers available by combining two 8-bit
/// registers.
#[derive(Debug, Eq, FromPrimitive, Hash, PartialEq)]
#[repr(u8)]
pub enum RegisterPair {
    /// The 16-bit register created by using the A and F registers
    Af = 0b11u8,

    /// The 16-bit register created by using the B and C registers
    Bc = 0b00u8,

    /// The 16-bit register created by using the D and E registers
    De = 0b01u8,

    /// The 16-bit register created by using the H and L registers
    Hl = 0b10u8,
}

impl RegisterPair {
    pub fn to_registers(&self) -> (Register, Register) {
        match self {
            RegisterPair::Af => (Register::A, Register::A),
            RegisterPair::Bc => (Register::C, Register::B),
            RegisterPair::De => (Register::E, Register::D),
            RegisterPair::Hl => (Register::L, Register::H),
        }
    }
}
