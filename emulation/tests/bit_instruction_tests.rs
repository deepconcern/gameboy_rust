use emulation::{
    flag::Flag,
    instruction::bit_instructions::{
        BIT_COMPLEMENT_OF_HL_LOCATION,
        BIT_COMPLEMENT_OF_REGISTER,
        RESET_BIT_OF_HL_LOCATION,
        RESET_BIT_OF_REGISTER,
        SET_BIT_OF_HL_LOCATION,
        SET_BIT_OF_REGISTER,
    },
    opcode::Opcode,
    Emulator,
};

mod common;

fn run_hl_location(opcode: u8, value: u8) -> Emulator {
    let mut emulator = common::setup_read_hl_location(opcode, value, true);

    emulator.process_opcode().unwrap();
    emulator.process_cycles();
    emulator.process_opcode().unwrap();

    emulator
}

fn run_register(opcode: u8, value: u8) -> Emulator {
    let register = opcode.parse_register(0b00_000_111).unwrap();

    let mut emulator = common::prefixed_emulator(opcode);

    emulator.set_register(register, value);

    emulator.process_opcode().unwrap();
    emulator.process_cycles();
    emulator.process_opcode().unwrap();

    emulator
}

mod bit_complement_of_hl_location {
    use super::*;

    #[test]
    fn cycles() {
        for opcode in BIT_COMPLEMENT_OF_HL_LOCATION.opcodes() {
            let emulator = super::run_hl_location(opcode, 0b10_101_010);

            assert_eq!(emulator.cycles(), 2);
        }
    }

    #[test]
    fn simple() {
        for opcode in BIT_COMPLEMENT_OF_HL_LOCATION.opcodes() {
            let bit_index = opcode.parse_bit(0b00_111_000);

            let emulator = super::run_hl_location(opcode, 0b10_101_010);

            assert_eq!(emulator.flag(Flag::Z), bit_index % 2 == 0);
            assert!(emulator.flag(Flag::H));
            assert!(!emulator.flag(Flag::N));
        }
    }
}

mod bit_complement_of_register {
    use super::*;

    #[test]
    fn cycles() {
        for opcode in BIT_COMPLEMENT_OF_REGISTER.opcodes() {
            let emulator = super::run_register(opcode, 0b10_101_010);

            assert_eq!(emulator.cycles(), 1);
        }
    }

    #[test]
    fn simple() {
        for opcode in BIT_COMPLEMENT_OF_REGISTER.opcodes() {
            let bit_index = opcode.parse_bit(0b00_111_000);

            let emulator = super::run_register(opcode, 0b10_101_010);

            assert_eq!(emulator.flag(Flag::Z), bit_index % 2 == 0);
            assert!(emulator.flag(Flag::H));
            assert!(!emulator.flag(Flag::N));
        }
    }
}

mod reset_bit_of_hl_location {
    use super::*;

    #[test]
    fn cycles() {
        for opcode in RESET_BIT_OF_HL_LOCATION.opcodes() {
            let emulator = super::run_hl_location(opcode, 0xff);

            assert_eq!(emulator.cycles(), 3);
        }
    }

    #[test]
    fn simple() {
        for opcode in RESET_BIT_OF_HL_LOCATION.opcodes() {
            let bit_index = opcode.parse_bit(0b00_111_000);

            let emulator = super::run_hl_location(opcode, 0xff);

            assert_eq!(emulator.memory_location(emulator.hl()), !(1u8 << bit_index));
        }
    }
}

mod reset_bit_of_register {
    use super::*;

    #[test]
    fn cycles() {
        for opcode in RESET_BIT_OF_REGISTER.opcodes() {
            let emulator = super::run_register(opcode, 0xff);

            assert_eq!(emulator.cycles(), 1);
        }
    }

    #[test]
    fn simple() {
        for opcode in RESET_BIT_OF_REGISTER.opcodes() {
            let bit_index = opcode.parse_bit(0b00_111_000);
            let register = opcode.parse_register(0b00_000_111).unwrap();

            let emulator = super::run_register(opcode, 0xff);

            assert_eq!(emulator.register(&register), !(1u8 << bit_index));
        }
    }
}

mod set_bit_of_hl_location {
    use super::*;

    #[test]
    fn cycles() {
        for opcode in SET_BIT_OF_HL_LOCATION.opcodes() {
            let emulator = super::run_hl_location(opcode, 0x00);

            assert_eq!(emulator.cycles(), 3);
        }
    }

    #[test]
    fn simple() {
        for opcode in SET_BIT_OF_HL_LOCATION.opcodes() {
            let bit_index = opcode.parse_bit(0b00_111_000);

            let emulator = super::run_hl_location(opcode, 0x00);

            assert_eq!(emulator.memory_location(emulator.hl()), 1u8 << bit_index);
        }
    }
}

mod set_bit_of_register {
    use super::*;

    #[test]
    fn cycles() {
        for opcode in SET_BIT_OF_REGISTER.opcodes() {
            let emulator = super::run_register(opcode, 0x00);

            assert_eq!(emulator.cycles(), 1);
        }
    }

    #[test]
    fn simple() {
        for opcode in SET_BIT_OF_REGISTER.opcodes() {
            let bit_index = opcode.parse_bit(0b00_111_000);
            let register = opcode.parse_register(0b00_000_111).unwrap();

            let emulator = super::run_register(opcode, 0x00);

            assert_eq!(emulator.register(&register), 1u8 << bit_index);
        }
    }
}