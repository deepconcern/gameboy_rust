use std::{fmt::Display, ops::Index, vec::IntoIter};

use regex::Regex;

const REGISTER_ARGUMENT_VARIATIONS: [&'static str; 7] = [
    "111", // A
    "000", // B
    "001", // C
    "010", // D
    "011", // E
    // No F register
    "100", // H
    "101", // L
];

const REGISTER_PAIR_ARGUMENT_VARIATIONS: [&'static str; 4] = [
    "11", // AF
    "00", // BC
    "01", // DE
    "10", // HL
];

lazy_static! {
    static ref REGISTER_ARGUMENT: Regex = Regex::new(r"rrr|sss").unwrap();
    static ref REGISTER_PAIR_ARGUMENT: Regex = Regex::new(r"[01](rr|ss)|(rr|ss)[01]").unwrap();
}

pub fn opcode_variations(prefix: &str, arg1: &str, arg2: &str) -> Vec<u8> {
    let arg_variations = [arg1, arg2].into_iter().map(|arg| {
        if REGISTER_ARGUMENT.is_match(arg) {
            REGISTER_ARGUMENT_VARIATIONS.into_iter().collect::<Vec<&'static str>>()
        } else if REGISTER_PAIR_ARGUMENT.is_match(arg) {
            REGISTER_PAIR_ARGUMENT_VARIATIONS.into_iter().collect::<Vec<&'static str>>()
        } else {
            vec![arg]
        }
    }).collect::<Vec<Vec<&str>>>();

    let arg1_variations = &arg_variations[0];
    let arg2_variations = &arg_variations[1];

    let mut variations = Vec::new();

    for arg1_variation in arg1_variations {
        for arg2_variation in arg2_variations {
            let mut opcode_chars = Vec::new();

            for c in prefix.chars() {
                opcode_chars.push(c);
            };

            for c in arg1_variation.chars() {
                opcode_chars.push(c);
            };

            for c in arg2_variation.chars() {
                opcode_chars.push(c);
            };

            let variation = opcode_chars.iter().collect::<String>();

            variations.push(match u8::from_str_radix(&variation, 2) {
                Ok(value) => value,
                Err(e) => {
                    panic!("Failed to parse {}: {:?}", variation, e);
                }
            });
        }
    };

    variations
}

pub struct Opcode {
    arg1: String,
    arg2: String,
    prefix: String,
    pub variations: Vec<u8>,
}

impl Opcode {
    pub fn new(pattern: &str) -> Opcode {
        let trimmed_pattern = pattern.replace(" ", "");

        if trimmed_pattern.len() != 8 {
            panic!("invalid opcode");
        }

        
        let arg1 = String::from(&trimmed_pattern[2..5]);
        let arg2 = String::from(&trimmed_pattern[5..8]);
        let prefix = String::from(&trimmed_pattern[0..2]);
        let variations = opcode_variations(&prefix, &arg1, &arg2);

        Opcode{
            arg1,
            arg2,
            prefix,
            variations,
        }
    }

    pub fn len(&self) -> usize {
        self.variations.len()
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.prefix, self.arg1, self.arg2)
    }
}

impl Index<usize> for Opcode {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.variations[index]
    }
}

impl IntoIterator for Opcode {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.variations.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::Opcode;

    #[derive(Clone, Debug, Default)]
    struct Environment {}

    #[test]
    fn simple_opcode() {
        let opcode = Opcode::new("00 000 000");

        assert_eq!(opcode.len(), 1usize);
        assert_eq!(opcode[0], 0b00000000);
    }

    #[test]
    fn register_arguments() {
        // Given an opcode with one or more arguments
        let one_arg_opcode = Opcode::new("10 000 rrr");
        let two_arg_opcode = Opcode::new("11 rrr sss");

        // It will make variations based on the argument configuration
        assert_eq!(one_arg_opcode.len(), 7usize);
        assert_eq!(one_arg_opcode[0], 0b10000111);
        assert_eq!(one_arg_opcode[1], 0b10000000);
        assert_eq!(one_arg_opcode[2], 0b10000001);
        assert_eq!(one_arg_opcode[3], 0b10000010);
        assert_eq!(one_arg_opcode[4], 0b10000011);
        // No 0b10000110
        assert_eq!(one_arg_opcode[5], 0b10000100);
        assert_eq!(one_arg_opcode[6], 0b10000101);

        assert_eq!(two_arg_opcode.len(), 49usize);
        assert_eq!(two_arg_opcode[0], 0b11111111);
        assert_eq!(two_arg_opcode[6], 0b11111101);
        assert_eq!(two_arg_opcode[13], 0b11000101);
        assert_eq!(two_arg_opcode[20], 0b11001101);
        assert_eq!(two_arg_opcode[27], 0b11010101);
        assert_eq!(two_arg_opcode[34], 0b11011101);
        assert_eq!(two_arg_opcode[41], 0b11100101);
        assert_eq!(two_arg_opcode[48], 0b11101101);
    }
}