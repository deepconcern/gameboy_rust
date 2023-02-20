pub struct BitAddResult {
    pub carry: bool,
    pub half_carry: bool,
    pub value: u8,
}

pub fn bit_add(a: u8, b: u8, carry: bool) -> BitAddResult {
    let a_u16 = a as u16;
    let b_u16 = b as u16;

    let carry_u16 = u16::from(carry);

    let a_low_u16 = a_u16 & 0x000fu16;
    let b_low_u16 = b_u16 & 0x000fu16;

    let sum_low_u16 = a_low_u16 + b_low_u16 + carry_u16;

    let sum_u16 = a_u16 + b_u16 + carry_u16;

    BitAddResult {
        carry: sum_u16 > 0xffu16,
        half_carry: sum_low_u16 > 0x000f,
        value: sum_u16 as u8,
    }
}

pub struct BitSubtractResult {
    pub borrow: bool,
    pub half_borrow: bool,
    pub value: u8,
}

pub fn bit_subtract(a: u8, b: u8, borrow: bool) -> BitSubtractResult {
    let borrow_value = u8::from(borrow);

    let a_low = a & 0x0fu8;
    let b_low = b & 0x0fu8;

    BitSubtractResult {
        borrow: match a.checked_sub(b) {
            Some(c) => match c.checked_sub(borrow_value) {
                Some(_) => false,
                None => true,
            },
            None => true,
        },
        half_borrow: match a_low.checked_sub(b_low) {
            Some(c_low) => match c_low.checked_sub(borrow_value) {
                Some(_) => false,
                None => true,
            },
            None => true,
        },
        value: a.wrapping_sub(b).wrapping_sub(u8::from(borrow)),
    }
}

#[cfg(test)]
mod tests {
    mod bit_add {
        use super::super::{bit_add, BitAddResult};

        #[test]
        fn simple_add() {
            let BitAddResult {
                carry,
                half_carry,
                value,
            } = bit_add(0x01u8, 0x02u8, false);

            assert_eq!(carry, false);
            assert_eq!(half_carry, false);
            assert_eq!(value, 0x03u8);
        }

        #[test]
        fn add_resulting_in_carry() {
            let BitAddResult {
                carry,
                half_carry,
                value,
            } = bit_add(0xf0u8, 0x10u8, false);

            assert_eq!(carry, true);
            assert_eq!(half_carry, false);
            assert_eq!(value, 0x00u8);
        }

        #[test]
        fn add_resulting_in_half_carry() {
            let BitAddResult {
                carry,
                half_carry,
                value,
            } = bit_add(0x0fu8, 0x01u8, false);

            assert_eq!(carry, false);
            assert_eq!(half_carry, true);
            assert_eq!(value, 0x10u8);
        }

        #[test]
        fn add_with_carry() {
            let BitAddResult {
                carry,
                half_carry,
                value,
            } = bit_add(0x01u8, 0x02u8, true);

            assert_eq!(carry, false);
            assert_eq!(half_carry, false);
            assert_eq!(value, 0x04u8);
        }
    }

    mod bit_subtract {
        use super::super::{bit_subtract, BitSubtractResult};

        #[test]
        fn simple_subtract() {
            let BitSubtractResult {
                borrow,
                half_borrow,
                value,
            } = bit_subtract(0x03u8, 0x01u8, false);

            assert_eq!(borrow, false);
            assert_eq!(half_borrow, false);
            assert_eq!(value, 0x02u8);
        }

        #[test]
        fn subtract_resulting_in_borrow() {
            let BitSubtractResult {
                borrow,
                half_borrow,
                value,
            } = bit_subtract(0xe0u8, 0xf0u8, false);

            assert_eq!(borrow, true);
            assert_eq!(half_borrow, false);
            assert_eq!(value, 0xf0u8);
        }

        #[test]
        fn subtract_resulting_in_half_borrow() {
            let BitSubtractResult {
                borrow,
                half_borrow,
                value,
            } = bit_subtract(0x1eu8, 0x0fu8, false);

            assert_eq!(borrow, false);
            assert_eq!(half_borrow, true);
            assert_eq!(value, 0x0fu8);
        }

        #[test]
        fn subtract_with_borrow() {
            let BitSubtractResult {
                borrow,
                half_borrow,
                value,
            } = bit_subtract(0x03u8, 0x02u8, true);

            assert_eq!(borrow, false);
            assert_eq!(half_borrow, false);
            assert_eq!(value, 0x00u8);
        }
    }
}
