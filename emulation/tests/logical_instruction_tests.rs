mod common;

use emulation::{
    flag::Flag,
    instruction::logical_instructions::{
        AND_HL_LOCATION,
        AND_IMMEDIATE_N,
        AND_REGISTER,
        OR_HL_LOCATION,
        OR_IMMEDIATE_N,
        OR_REGISTER,
        XOR_HL_LOCATION,
        XOR_IMMEDIATE_N,
        XOR_REGISTER,
    },
    opcode::Opcode,
    register::Register,
    Emulator
};


fn and_flag_test(emulator: Emulator, zeroed: bool) {
    assert!(!emulator.flag(Flag::CY));
    assert!(emulator.flag(Flag::H));
    assert!(!emulator.flag(Flag::N));
    assert_eq!(emulator.flag(Flag::Z), zeroed);
}

fn or_flag_test(emulator: Emulator, zeroed: bool) {
    assert!(!emulator.flag(Flag::CY));
    assert!(!emulator.flag(Flag::H));
    assert!(!emulator.flag(Flag::N));
    assert_eq!(emulator.flag(Flag::Z), zeroed);
}

fn xor_flag_test(emulator: Emulator, zeroed: bool) {
    assert!(!emulator.flag(Flag::CY));
    assert!(!emulator.flag(Flag::H));
    assert!(!emulator.flag(Flag::N));
    assert_eq!(emulator.flag(Flag::Z), zeroed);
}

mod and_hl_location {
    use super::*;

    fn run(a: u8, b: u8) -> Emulator {
        let opcode = AND_HL_LOCATION.opcodes()[0];

        let mut emulator = common::setup_read_hl_location(opcode, b, false);

        emulator.set_a(a);

        emulator.process_opcode().unwrap();

        emulator
    }

    #[test]
    fn cycles() {
        let emulator = run(0x05, 0x09);

        assert_eq!(emulator.cycles(), 2);
    }

    #[test]
    fn simple() {
        let emulator = run(0x05, 0x09);

        assert_eq!(emulator.a(), 0x01);
        super::and_flag_test(emulator, false);
    }

    #[test]
    fn zeroed() {
        let emulator = run(0x01, 0x0e);

        assert_eq!(emulator.a(), 0x00);
        super::and_flag_test(emulator, true);
    }
}

mod and_immediate_n {
    use super::*;

    fn run(a: u8, b: u8) -> Emulator {
        let opcode = AND_IMMEDIATE_N.opcodes()[0];

        let mut emulator = common::setup_read_immediate_n(opcode, b, false);

        emulator.set_a(a);

        emulator.process_opcode().unwrap();

        emulator
    }

    #[test]
    fn cycles() {
        let emulator = run(0x05, 0x09);

        assert_eq!(emulator.cycles(), 2);
    }

    #[test]
    fn simple() {
        let emulator = run(0x05, 0x09);

        assert_eq!(emulator.a(), 0x01);
        super::and_flag_test(emulator, false);
    }

    #[test]
    fn zeroed() {
        let emulator = run(0x01, 0x0e);

        assert_eq!(emulator.a(), 0x00);
        super::and_flag_test(emulator, true);
    }
}

mod and_register {
    use super::*;

    fn run(opcode: u8, a: u8, b: u8) -> Emulator {
        let register = opcode.parse_register(0b00_000_111).unwrap();

        let mut emulator = common::simple_emulator(opcode);

        emulator.set_a(a);
        emulator.set_register(register, b);

        emulator.process_opcode().unwrap();

        emulator
    }

    #[test]
    fn cycles() {
        for opcode in AND_REGISTER.opcodes() {
            let emulator = run(opcode, 0x05, 0x09);

            assert_eq!(emulator.cycles(), 1);
        }
    }

    #[test]
    fn simple() {
        for opcode in AND_REGISTER.opcodes() {
            let emulator = run(opcode, 0x05, 0x09);

            let register = opcode.parse_register(0b00_000_111).unwrap();
            
            if register == Register::A {
                assert_eq!(emulator.a(), 0x09);
            } else {
                assert_eq!(emulator.a(), 0x01);
            };

            super::and_flag_test(emulator, false);
        }
    }

    #[test]
    fn zeroed() {
        for opcode in AND_REGISTER.opcodes() {
            let register = opcode.parse_register(0b00_000_111).unwrap();
            
            let emulator = if register == Register::A {
                run(opcode, 0x00, 0x00)
            } else {
                run(opcode, 0x01, 0x0e)
            };

            assert_eq!(emulator.a(), 0x00);
            super::and_flag_test(emulator, true);
        }
    }
}

mod or_hl_location {
    use super::*;

    fn run(a: u8, b: u8) -> Emulator {
        let opcode = OR_HL_LOCATION.opcodes()[0];

        let mut emulator = common::setup_read_hl_location(opcode, b, false);

        emulator.set_a(a);

        emulator.process_opcode().unwrap();

        emulator
    }

    #[test]
    fn cycles() {
        let emulator = run(0x05, 0x09);

        assert_eq!(emulator.cycles(), 2);
    }

    #[test]
    fn simple() {
        let emulator = run(0x05, 0x09);

        assert_eq!(emulator.a(), 0x0d);
        super::or_flag_test(emulator, false);
    }

    #[test]
    fn zeroed() {
        let emulator = run(0x00, 0x00);

        assert_eq!(emulator.a(), 0x00);
        super::or_flag_test(emulator, true);
    }
}

mod or_immediate_n {
    use super::*;

    fn run(a: u8, b: u8) -> Emulator {
        let opcode = OR_IMMEDIATE_N.opcodes()[0];

        let mut emulator = common::setup_read_immediate_n(opcode, b, false);

        emulator.set_a(a);

        emulator.process_opcode().unwrap();

        emulator
    }

    #[test]
    fn cycles() {
        let emulator = run(0x05, 0x09);

        assert_eq!(emulator.cycles(), 2);
    }

    #[test]
    fn simple() {
        let emulator = run(0x05, 0x09);

        assert_eq!(emulator.a(), 0x0d);
        super::or_flag_test(emulator, false);
    }

    #[test]
    fn zeroed() {
        let emulator = run(0x00, 0x00);

        assert_eq!(emulator.a(), 0x00);
        super::or_flag_test(emulator, true);
    }
}

mod or_register {
    use super::*;

    fn run(opcode: u8, a: u8, b: u8) -> Emulator {
        let register = opcode.parse_register(0b00_000_111).unwrap();
        
        let mut emulator = common::simple_emulator(opcode);

        emulator.set_a(a);
        emulator.set_register(register, b);

        emulator.process_opcode().unwrap();

        emulator
    }

    #[test]
    fn cycles() {
        for opcode in OR_REGISTER.opcodes() {
            let emulator = run(opcode, 0x05, 0x09);

            assert_eq!(emulator.cycles(), 1);
        }
    }

    #[test]
    fn simple() {
        for opcode in OR_REGISTER.opcodes() {
            let emulator = run(opcode, 0x05, 0x09);

            let register = opcode.parse_register(0b00_000_111).unwrap();
            
            if register == Register::A {
                assert_eq!(emulator.a(), 0x09);
            } else {
                assert_eq!(emulator.a(), 0x0d);
            };
            
            super::or_flag_test(emulator, false);
        }
    }

    #[test]
    fn zeroed() {
        for opcode in OR_REGISTER.opcodes() {
            let emulator = run(opcode, 0x00, 0x00);

            assert_eq!(emulator.a(), 0x00);
            super::or_flag_test(emulator, true);
        }
    }
}

mod xor_hl_location {
    use super::*;

    fn run(a: u8, b: u8) -> Emulator {
        let opcode = XOR_HL_LOCATION.opcodes()[0];

        let mut emulator = common::setup_read_hl_location(opcode, b, false);

        emulator.set_a(a);

        emulator.process_opcode().unwrap();

        emulator
    }

    #[test]
    fn cycles() {
        let emulator = run(0x05, 0x09);

        assert_eq!(emulator.cycles(), 2);
    }

    #[test]
    fn simple() {
        let emulator = run(0x05, 0x09);

        assert_eq!(emulator.a(), 0x0c);
        super::xor_flag_test(emulator, false);
    }

    #[test]
    fn zeroed() {
        let emulator = run(0x01, 0x01);

        assert_eq!(emulator.a(), 0x00);
        super::xor_flag_test(emulator, true);
    }
}

mod xor_immediate_n {
    use super::*;

    fn run(a: u8, b: u8) -> Emulator {
        let opcode = XOR_IMMEDIATE_N.opcodes()[0];

        let mut emulator = common::setup_read_immediate_n(opcode, b, false);

        emulator.set_a(a);

        emulator.process_opcode().unwrap();

        emulator
    }

    #[test]
    fn cycles() {
        let emulator = run(0x05, 0x09);

        assert_eq!(emulator.cycles(), 2);
    }

    #[test]
    fn simple() {
        let emulator = run(0x05, 0x09);

        assert_eq!(emulator.a(), 0x0c);
        super::xor_flag_test(emulator, false);
    }

    #[test]
    fn zeroed() {
        let emulator = run(0x01, 0x01);

        assert_eq!(emulator.a(), 0x00);
        super::xor_flag_test(emulator, true);
    }
}

mod xor_register {
    use super::*;

    fn run(opcode: u8, a: u8, b: u8) -> Emulator {
        let register = opcode.parse_register(0b00_000_111).unwrap();

        let mut emulator = common::simple_emulator(opcode);

        emulator.set_a(a);
        emulator.set_register(register, b);

        emulator.process_opcode().unwrap();

        emulator
    }

    #[test]
    fn cycles() {
        for opcode in XOR_REGISTER.opcodes() {
            let emulator = run(opcode, 0x05, 0x09);

            assert_eq!(emulator.cycles(), 1);
        }
    }

    #[test]
    fn simple() {
        for opcode in XOR_REGISTER.opcodes() {
            let emulator = run(opcode, 0x05, 0x09);

            let register = opcode.parse_register(0b00_000_111).unwrap();
            
            if register == Register::A {
                continue;
            } else {
                assert_eq!(emulator.a(), 0x0c);
            };
            
            super::xor_flag_test(emulator, false);
        }
    }

    #[test]
    fn zeroed() {
        for opcode in XOR_REGISTER.opcodes() {
            let emulator = run(opcode, 0x01, 0x01);

            assert_eq!(emulator.a(), 0x00);
            super::xor_flag_test(emulator, true);
        }
    }
}