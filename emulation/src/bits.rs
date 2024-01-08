use num_traits::{Bounded, PrimInt, ops::{overflowing::{OverflowingAdd, OverflowingSub}, wrapping::WrappingAdd}, Unsigned, Signed};

pub trait SignedInt: Bounded + PrimInt + OverflowingAdd + OverflowingSub + WrappingAdd + Signed {}

impl SignedInt for i8 {}

pub trait UnsignedInt: Bounded + PrimInt + OverflowingAdd + OverflowingSub + WrappingAdd + Unsigned {}

impl UnsignedInt for u8 {}
impl UnsignedInt for u16 {}

pub fn bit_add<U: UnsignedInt>(a: U, b: U, carry: bool) -> (U, bool, bool) {
    let carry_value = if carry { U::one() } else { U::zero() };
    

    let half_value = U::max_value() >> (U::zero().count_zeros() / 2) as usize;

    let has_half_carry: bool = (a & half_value) + (b & half_value) + carry_value > half_value;

    match a.overflowing_add(&b) {
        (sum, has_carry) => if has_carry {
            (sum.overflowing_add(&carry_value).0, true, has_half_carry)
        } else {
            let (sum, has_carry) = sum.overflowing_add(&carry_value);

            (sum, has_carry, has_half_carry)
        }
    }
}

pub fn bit_subtract<U: UnsignedInt>(a: U, b: U, borrow: bool) -> (U, bool, bool) {
    let borrow_value = if borrow { U::one() } else { U::zero() };

    let half_value = U::max_value() >> (U::zero().count_zeros() / 2) as usize;

    let has_half_borrow = (a & half_value) < (b.wrapping_add(&borrow_value)) & half_value;

    match a.overflowing_sub(&b) {
        (dif, has_borrow) => if has_borrow {
            (dif.overflowing_sub(&borrow_value).0, true, has_half_borrow)
        } else {
            let (dif, has_borrow) = dif.overflowing_sub(&borrow_value);

            (dif, has_borrow, has_half_borrow)
        }
    }
}

#[cfg(test)]
mod tests {
    mod bit_add {
        use super::super::bit_add;

        #[test]
        fn simple_add() {
            let (value, carry, half_carry) = bit_add(0x00feu16, 0x0001u16, false);

            assert_eq!(carry, false);
            assert_eq!(half_carry, false);
            assert_eq!(value, 0x00ffu16);

            let (value, carry, half_carry) = bit_add(0x01u8, 0x02u8, false);

            assert_eq!(carry, false);
            assert_eq!(half_carry, false);
            assert_eq!(value, 0x03u8);
        }

        #[test]
        fn add_resulting_in_carry() {
            let (value, carry, half_carry) = bit_add(0xf000u16, 0x1000u16, false);

            assert_eq!(carry, true);
            assert_eq!(half_carry, false);
            assert_eq!(value, 0x0000u16);
            
            let (value, carry, half_carry) = bit_add(0xf0u8, 0x10u8, false);

            assert_eq!(carry, true);
            assert_eq!(half_carry, false);
            assert_eq!(value, 0x00u8);
        }

        #[test]
        fn add_resulting_in_half_carry() {
            let (value, carry, half_carry) = bit_add(0x0fu8, 0x01u8, false);

            assert_eq!(carry, false);
            assert_eq!(half_carry, true);
            assert_eq!(value, 0x10u8);
            
            let (value, carry, half_carry) = bit_add(0x01u8, 0x02u8, true);

            assert_eq!(carry, false);
            assert_eq!(half_carry, false);
            assert_eq!(value, 0x04u8);
        }
    }

    mod bit_subtract {
        use super::super::bit_subtract;

        #[test]
        fn simple_subtract() {
            let (value, borrow, half_borrow) = bit_subtract(0x03u8, 0x01u8, false);

            assert_eq!(borrow, false);
            assert_eq!(half_borrow, false);
            assert_eq!(value, 0x02u8);

            let (value, borrow, half_borrow) = bit_subtract(0x0003u16, 0x0001u16, false);

            assert_eq!(borrow, false);
            assert_eq!(half_borrow, false);
            assert_eq!(value, 0x0002u16);
        }

        #[test]
        fn subtract_resulting_in_borrow() {
            let (value, borrow, half_borrow) = bit_subtract(0xe0u8, 0xf0u8, false);

            assert_eq!(borrow, true);
            assert_eq!(half_borrow, false);
            assert_eq!(value, 0xf0u8);

            let (value, borrow, half_borrow) = bit_subtract(0xe000u16, 0xf000u16, false);

            assert_eq!(borrow, true);
            assert_eq!(half_borrow, false);
            assert_eq!(value, 0xf000u16);
        }

        #[test]
        fn subtract_resulting_in_half_borrow() {
            let (value, borrow, half_borrow) = bit_subtract(0x1eu8, 0x0fu8, false);

            assert_eq!(borrow, false);
            assert_eq!(half_borrow, true);
            assert_eq!(value, 0x0fu8);
            
            let (value, borrow, half_borrow) = bit_subtract(0x0100u16, 0x0080u16, false);

            assert_eq!(borrow, false);
            assert_eq!(half_borrow, true);
            assert_eq!(value, 0x0080u16);
        }

        #[test]
        fn subtract_with_borrow() {
            let (value, borrow, half_borrow) = bit_subtract(0x03u8, 0x02u8, true);

            assert_eq!(borrow, false);
            assert_eq!(half_borrow, false);
            assert_eq!(value, 0x00u8);

            let (value, borrow, half_borrow) = bit_subtract(0x03u16, 0x02u16, true);

            assert_eq!(borrow, false);
            assert_eq!(half_borrow, false);
            assert_eq!(value, 0x00u16);
        }
    }
}
