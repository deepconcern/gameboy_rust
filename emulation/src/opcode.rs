use std::collections::HashSet;

use num::FromPrimitive;

use crate::{register::{Register, RegisterPair}, condition::Condition};

#[derive(Debug, PartialEq)]
pub enum OpcodeError {
    ConditionParse(u8),
    PageParse(u8),
    RegisterPairParse(u8),
    RegisterParse(u8),
}

pub trait Opcode {
    fn parse_bit(&self, mask: u8) -> usize;
    fn parse_condition(&self, mask: u8) -> Result<Condition, OpcodeError>;
    fn parse_page(&self, mask: u8) -> Result<u16, OpcodeError>;
    fn parse_register(&self, mask: u8) -> Result<Register, OpcodeError>;
    fn parse_register_pair(&self, mask: u8) -> Result<RegisterPair, OpcodeError>;
}

impl Opcode for u8 {
    fn parse_bit(&self, mask: u8) -> usize {
        (self & mask) as usize >> mask.trailing_zeros() as usize
    }

    fn parse_condition(&self, mask: u8) -> Result<Condition, OpcodeError> {
        let argument = (self & mask) >> mask.trailing_zeros() as usize;

        Condition::from_u8(argument).ok_or(OpcodeError::ConditionParse(argument))
    }

    fn parse_page(&self, mask: u8) -> Result<u16, OpcodeError> {
        let argument = (self & mask) >> mask.trailing_zeros() as usize;

        match argument {
            0 => Ok(0x0000u16),
            1 => Ok(0x0008u16),
            2 => Ok(0x0010u16),
            3 => Ok(0x0018u16),
            4 => Ok(0x0020u16),
            5 => Ok(0x0028u16),
            6 => Ok(0x0030u16),
            7 => Ok(0x0038u16),
            _ => Err(OpcodeError::PageParse(argument)),
        }
    }

    fn parse_register(&self, mask: u8) -> Result<Register, OpcodeError> {
        let argument = (self & mask) >> mask.trailing_zeros() as usize;

        Register::from_u8(argument).ok_or(OpcodeError::RegisterParse(argument))
    }

    fn parse_register_pair(&self, mask: u8) -> Result<RegisterPair, OpcodeError> {
        let argument = (self & mask) >> mask.trailing_zeros() as usize;

        RegisterPair::from_u8(argument).ok_or(OpcodeError::RegisterPairParse(argument))
    }
}

pub const TWO_BIT_VARIATIONS: [&str; 4] = ["00", "01", "10", "11"];
pub const THREE_BIT_VARIATIONS: [&str; 8] = ["000", "001", "010", "011", "100", "101", "110", "111"];

pub const REGISTER_PATTERN_A: &str = "rrr";
pub const REGISTER_PATTERN_B: &str = "qqq";
pub const REGISTER_VARIATIONS: [&str; 7] = ["000", "001", "010", "011", "100", "101", "111"];

pub const REGISTER_PAIR_PATTERN_A: &str = "ss";
pub const REGISTER_PAIR_PATTERN_B: &str = "dd";

pub const BIT_PATTERN: &str = "bbb";

pub const CONDITION_PATTERN: &str = "cc";

pub const MEMORY_PATTERN: &str = "ttt";

fn process(opcode_strings: &mut Vec<String>, opcode_string: &str, pattern: &str, variations: Vec<&str>) -> bool {
    if opcode_string.contains(pattern) {
        for variation in variations {
            opcode_strings.push(opcode_string.replace(pattern, variation));
        }

        true
    } else {
        false
    }
}

pub trait OpcodePattern: Into<Vec<u8>> {
    fn into(&self) -> Vec<u8> {
        self.opcodes()
    }

    fn opcodes(&self) -> Vec<u8>;
}

impl OpcodePattern for &str {
    fn opcodes(&self) -> Vec<u8> {
        let trimmed_self = self.replace(" ", "");

        let mut opcode_strings = vec![trimmed_self];
        let mut opcodes = HashSet::new();

        // Parse registers
        while !opcode_strings.is_empty() {
            let opcode_string = opcode_strings.pop().unwrap();

            // Registers

            if process(&mut opcode_strings, &opcode_string, REGISTER_PATTERN_A, REGISTER_VARIATIONS.into()) {
                continue;
            }

            if process(&mut opcode_strings, &opcode_string, REGISTER_PATTERN_B, REGISTER_VARIATIONS.into()) {
                continue;
            }

            // Register pairs

            if process(&mut opcode_strings, &opcode_string, REGISTER_PAIR_PATTERN_A, TWO_BIT_VARIATIONS.into()) {
                continue;
            }

            if process(&mut opcode_strings, &opcode_string, REGISTER_PAIR_PATTERN_B, TWO_BIT_VARIATIONS.into()) {
                continue;
            }

            // Conditional arguments

            if process(&mut opcode_strings, &opcode_string, CONDITION_PATTERN, TWO_BIT_VARIATIONS.into()) {
                continue;
            }

            // Bit arguments

            if process(&mut opcode_strings, &opcode_string, BIT_PATTERN, THREE_BIT_VARIATIONS.into()) {
                continue;
            }

            // Memory arguments

            if process(&mut opcode_strings, &opcode_string, MEMORY_PATTERN, THREE_BIT_VARIATIONS.into()) {
                continue;
            }

            match u8::from_str_radix(&opcode_string, 2).ok() {
                Some(opcode) => opcodes.insert(opcode),
                None => panic!("Failed to parse opcode '{}'", opcode_string)
            };
        }

        opcodes.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    mod opcode {
        use crate::{register::{Register, RegisterPair}, opcode::{Opcode, OpcodeError}};

        #[test]
        fn parse_register() {
            let mask = 0b00_111_000u8;

            assert_eq!(0b00_111_000u8.parse_register(mask).unwrap(), Register::A);
            assert_eq!(0b00_000_000u8.parse_register(mask).unwrap(), Register::B);
            assert_eq!(0b00_001_000u8.parse_register(mask).unwrap(), Register::C);
            assert_eq!(0b00_010_000u8.parse_register(mask).unwrap(), Register::D);
            assert_eq!(0b00_011_000u8.parse_register(mask).unwrap(), Register::E);
            assert_eq!(0b00_100_000u8.parse_register(mask).unwrap(), Register::H);
            assert_eq!(0b00_101_000u8.parse_register(mask).unwrap(), Register::L);
            assert_eq!(0b00_110_000u8.parse_register(mask), Err(OpcodeError::RegisterParse(0b110u8)));
        }

        #[test]
        fn parse_register_pair() {
            let mask = 0b00_110_000u8;

            assert_eq!(0b00_110_000u8.parse_register_pair(mask).unwrap(), RegisterPair::Af);
            assert_eq!(0b00_000_000u8.parse_register_pair(mask).unwrap(), RegisterPair::Bc);
            assert_eq!(0b00_010_000u8.parse_register_pair(mask).unwrap(), RegisterPair::De);
            assert_eq!(0b00_100_000u8.parse_register_pair(mask).unwrap(), RegisterPair::Hl);

            assert_eq!(0b00_111_000u8.parse_register_pair(0b00_111_000), Err(OpcodeError::RegisterPairParse(0b111u8)));
        }
    }

    mod opcode_pattern {
        use std::collections::HashSet;

        use crate::opcode::OpcodePattern;

        #[test]
        fn multiple_register_pairs() {
            let opcodes: HashSet<u8> = "01 ss0 dd0".opcodes().into_iter().collect();

            assert_eq!(opcodes, HashSet::from([
                0b01_000_000,
                0b01_010_000,
                0b01_100_000,
                0b01_110_000,
                0b01_000_010,
                0b01_010_010,
                0b01_100_010,
                0b01_110_010,
                0b01_000_100,
                0b01_010_100,
                0b01_100_100,
                0b01_110_100,
                0b01_000_110,
                0b01_010_110,
                0b01_100_110,
                0b01_110_110,
            ]));
        }

        #[test]
        fn multiple_registers() {
            let opcodes: HashSet<u8> = "01 rrr qqq".opcodes().into_iter().collect();

            assert_eq!(opcodes, HashSet::from([
                0b01_000_000,
                0b01_001_000,
                0b01_010_000,
                0b01_011_000,
                0b01_100_000,
                0b01_101_000,
                0b01_111_000,
                0b01_000_001,
                0b01_001_001,
                0b01_010_001,
                0b01_011_001,
                0b01_100_001,
                0b01_101_001,
                0b01_111_001,
                0b01_000_010,
                0b01_001_010,
                0b01_010_010,
                0b01_011_010,
                0b01_100_010,
                0b01_101_010,
                0b01_111_010,
                0b01_000_011,
                0b01_001_011,
                0b01_010_011,
                0b01_011_011,
                0b01_100_011,
                0b01_101_011,
                0b01_111_011,
                0b01_000_100,
                0b01_001_100,
                0b01_010_100,
                0b01_011_100,
                0b01_100_100,
                0b01_101_100,
                0b01_111_100,
                0b01_000_101,
                0b01_001_101,
                0b01_010_101,
                0b01_011_101,
                0b01_100_101,
                0b01_101_101,
                0b01_111_101,
                0b01_000_111,
                0b01_001_111,
                0b01_010_111,
                0b01_011_111,
                0b01_100_111,
                0b01_101_111,
                0b01_111_111,
            ]));
        }

        #[test]
        fn no_pattern() {
            let opcodes: HashSet<u8> = "10 000 110".opcodes().into_iter().collect();

            assert_eq!(opcodes, HashSet::from([0b10_000_110]));
        }

        #[test]
        fn single_register() {
            let opcodes: HashSet<u8> = "01 rrr 110".opcodes().into_iter().collect();

            assert_eq!(opcodes, HashSet::from([
                0b01_000_110,
                0b01_001_110,
                0b01_010_110,
                0b01_011_110,
                0b01_100_110,
                0b01_101_110,
                0b01_111_110,
            ]));

            let opcodes: HashSet<u8> = "01 qqq 110".opcodes().into_iter().collect();

            assert_eq!(opcodes, HashSet::from([
                0b01_000_110,
                0b01_001_110,
                0b01_010_110,
                0b01_011_110,
                0b01_100_110,
                0b01_101_110,
                0b01_111_110,
            ]));
        }

        #[test]
        fn single_register_pair() {
            let opcodes: HashSet<u8> = "01 ss0 110".opcodes().into_iter().collect();

            assert_eq!(opcodes, HashSet::from([
                0b01_000_110,
                0b01_010_110,
                0b01_100_110,
                0b01_110_110,
            ]));

            let opcodes: HashSet<u8> = "01 dd0 110".opcodes().into_iter().collect();

            assert_eq!(opcodes, HashSet::from([
                0b01_000_110,
                0b01_010_110,
                0b01_100_110,
                0b01_110_110,
            ]));
        }
    }
}