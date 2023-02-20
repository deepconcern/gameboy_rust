#[derive(FromPrimitive)]
#[repr(u8)]

pub enum Flag {
    Z = 0b10000000,
    N = 0b01000000,
    H = 0b00100000,
    CY = 0b00010000,
}
