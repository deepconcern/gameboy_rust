use crate::{emulator::Emulator, flag::Flag};

#[derive(FromPrimitive)]
#[repr(u8)]
pub enum Condition {
    C = 0b11,
    Nc = 0b10,
    Nz = 0b00,
    Z = 0b01,
}

impl Condition {
    pub fn check(&self, emulator: &Emulator) -> bool {
        match self {
            Condition::C => emulator.flag(Flag::CY),
            Condition::Nc => !emulator.flag(Flag::CY),
            Condition::Nz => !emulator.flag(Flag::Z),
            Condition::Z => emulator.flag(Flag::Z),
        }
    }
}