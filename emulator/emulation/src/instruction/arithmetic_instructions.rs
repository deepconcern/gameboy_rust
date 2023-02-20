use crate::flag::Flag;
use crate::processor_state::ProcessorState;
use crate::register::{Register, RegisterPair};

use super::instruction::{Instruction, InstructionError, parse_register_argument};

/// ADD A, (HL)
///
/// A <- A + (HL)
///
/// Adds the contents of memory specified by the contents of register pair HL
/// to the contents of register A and stores the results in register A.
#[instruction(cycles = 2, name = "ADD A, (HL)", opcode_pattern = "10 000 110")]
fn add_hl_location_to_a(processor_state: &mut ProcessorState, opcode: u8) -> Result<(), InstructionError> {
    let location = processor_state.get_register_pair(&RegisterPair::Hl);

    let value = processor_state.read(location)?;

    processor_state.add_to_a(value, false);

    Ok(())
}

/// ADD A, n
///
/// A <- A + n
///
/// Adds 8-bit immediate operand n to the contents of register A and stores the
/// results in register A.
#[instruction(cycles = 2, name = "ADD A, n", opcode_pattern = "11 000 110")]
fn add_immediate_n_to_a(processor_state: &mut ProcessorState, opcode: u8) -> Result<(), InstructionError> {
    let n = processor_state.get_immediate_n()?;

    processor_state.add_to_a(n, false);

    Ok(())
}

/// ADD A, r
///
/// A <- A + r
///
/// Adds the contents of register r to those of register A and stores the
/// results in register A.
#[instruction(cycles = 1, name = "ADD A, r", opcode_pattern = "10 000 rrr")]
pub fn add_register_to_a(processor_state: &mut ProcessorState, opcode: u8) -> Result<(), InstructionError> {
    let register = parse_register_argument(&opcode, 0b00000111)?;

    let a_value = processor_state.registers[&Register::A];
    let other_value = processor_state.registers[&register];

    let computed_value = processor_state.add(a_value, other_value, false);

    processor_state
        .registers
        .insert(Register::A, computed_value);

    Ok(())
}

/// ADC A, (HL)
///
/// A <- A + (HL) + CY
///
/// Adds the contents of register pair HL and CY to the contents of register
/// A and stores the results in register A.
#[instruction(cycles = 2, name = "ADC A, (HL)", opcode_pattern = "10 001 110")]
pub fn add_with_cary_hl_location_to_a(processor_state: &mut ProcessorState, opcode: u8) -> Result<(), InstructionError> {
    let location = processor_state.get_register_pair(&RegisterPair::Hl);

    let value = processor_state.read(location)?;

    processor_state.add_to_a(value, true);

    Ok(())
}

/// ADC A, n
///
/// A <- A + n + CY
///
/// Adds 8-bit immediate operand n and CY to the contents of register A and
/// stores the results in register A.
#[instruction(cycles = 2, name = "ADC A, n", opcode_pattern = "11 001 110")]
pub fn add_with_carry_immediate_n_to_a(processor_state: &mut ProcessorState, opcode: u8) -> Result<(), InstructionError> {
    let n = processor_state.get_immediate_n()?;

    processor_state.add_to_a(n, true);

    Ok(())
}



#[cfg(test)]
mod tests {
    mod add_hl_location_to_a {
        use crate::{
            instruction::Instruction,
            processor_state::ProcessorState,
            register::Register,
        };

        use super::super::add_hl_location_to_a;

        #[test]
        fn operation() {
            let instruction = build_instruction!(add_hl_location_to_a);

            // Only one variation
            let opcode = instruction.variations()[0];

            let mut processor_state = ProcessorState::new();

            let h = 0x02u8;
            let l = 0x03u8;
            let hl = u16::from_le_bytes([l, h]);

            processor_state.registers.insert(Register::A, 0x01u8);
            processor_state.registers.insert(Register::H, h);
            processor_state.registers.insert(Register::L, l);

            processor_state.write(hl, 0x04u8).unwrap();
            
            instruction.operation(&mut processor_state, opcode).unwrap();

            let actual_value = processor_state.registers[&Register::A];
            let expected_value = 0x04u8 + 0x01u8;

            assert_eq!(actual_value, expected_value);
        }

        #[test]
        fn variations() {
            let instruction = build_instruction!(add_hl_location_to_a);

            assert_eq!(instruction.variations().len(), 1);
        }
    }

    mod add_immediate_n_to_a {
        use crate::{
            instruction::Instruction,
            processor_state::ProcessorState,
            register::Register,
        };

        use super::super::add_immediate_n_to_a;

        #[test]
        fn operation() {
            let instruction = build_instruction!(add_immediate_n_to_a);

            // Only one variation
            let opcode = instruction.variations()[0];

            let mut processor_state = ProcessorState::new();

            processor_state.write(processor_state.program_counter, 0x03u8).unwrap();

            processor_state.registers.insert(Register::A, 0x01u8);

            instruction.operation(&mut processor_state, opcode).unwrap();

            let actual_value = processor_state.registers[&Register::A];
            let expected_value = 0x03u8 + 0x01u8;

            assert_eq!(actual_value, expected_value);
        }

        #[test]
        fn variations() {
            let instruction = build_instruction!(add_immediate_n_to_a);

            assert_eq!(instruction.variations().len(), 1);
        }
    }

    mod add_register_to_a {
        use num::FromPrimitive;

        use crate::{
            flag::Flag,
            instruction::Instruction,
            processor_state::ProcessorState,
            register::Register,
        };

        use super::super::add_register_to_a;

        #[test]
        fn operation() {
            let instruction = build_instruction!(add_register_to_a);

            // Test each variation
            for opcode in opcode_variations!(add_register_to_a) {
                let argument = opcode & 0b00000111u8;
                let source_register = Register::from_u8(argument).unwrap();

                let mut processor_state = ProcessorState::new();

                processor_state.registers.insert(Register::A, 1u8);
                processor_state.registers.insert(Register::B, 1u8);
                processor_state.registers.insert(Register::C, 2u8);
                processor_state.registers.insert(Register::D, 3u8);
                processor_state.registers.insert(Register::E, 4u8);
                processor_state.registers.insert(Register::F, 5u8);
                processor_state.registers.insert(Register::H, 6u8);
                processor_state.registers.insert(Register::L, 7u8);

                instruction.operation(&mut processor_state, opcode).unwrap();

                let actual_value = processor_state.registers[&Register::A];
                let expected_value = if source_register == Register::A {
                    2u8
                } else {
                    processor_state.registers[&source_register] + 1u8
                };

                // It should add the values of the A and source register
                assert_eq!(
                    actual_value, expected_value,
                    "Failed for source {:?}",
                    source_register
                );
                assert_eq!(
                    processor_state.flag_enabled(Flag::CY),
                    false,
                    "Failed for source {:?}",
                    source_register
                );
                assert_eq!(
                    processor_state.flag_enabled(Flag::H),
                    false,
                    "Failed for source {:?}",
                    source_register
                );
                assert_eq!(
                    processor_state.flag_enabled(Flag::N),
                    false,
                    "Failed for source {:?}",
                    source_register
                );
                assert_eq!(
                    processor_state.flag_enabled(Flag::Z),
                    false,
                    "Failed for source {:?}",
                    source_register
                );
            }
        }

        #[test]
        fn variations() {
            let instruction = build_instruction!(add_register_to_a);

            assert_eq!(instruction.variations().len(), 7);
        }
    }
}
