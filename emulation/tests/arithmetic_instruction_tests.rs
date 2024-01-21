mod common;

use emulation::{
    flag::Flag,
    instruction::arithmetic_instructions::{
        ADD_HL_LOCATION_TO_A,
        ADD_IMMEDIATE_E_TO_SP,
        ADD_IMMEDIATE_N_TO_A,
        ADD_REGISTER_PAIR_TO_HL,
        ADD_REGISTER_TO_A,
        ADD_WITH_CARRY_HL_LOCATION_TO_A,
        ADD_WITH_CARRY_IMMEDIATE_N_TO_A,
        ADD_WITH_CARRY_REGISTER_TO_A,
        COMPARE_HL_LOCATION_WITH_A,
        COMPARE_IMMEDIATE_N_WITH_A,
        COMPARE_REGISTER_WITH_A,
        DEC_HL_LOCATION,
        DEC_REGISTER,
        DEC_REGISTER_PAIR,
        INC_HL_LOCATION,
        INC_REGISTER,
        INC_REGISTER_PAIR,
        SUB_HL_LOCATION_FROM_A,
        SUB_IMMEDIATE_N_FROM_A,
        SUB_REGISTER_FROM_A,
        SUB_WITH_CARRY_HL_LOCATION_FROM_A,
        SUB_WITH_CARRY_IMMEDIATE_N_FROM_A,
        SUB_WITH_CARRY_REGISTER_FROM_A,
    },
    opcode::Opcode,
    register::Register,
    Emulator
};

fn test_add_simple_flags(emulator: Emulator) {
    assert!(!emulator.flag(Flag::CY));
    assert!(!emulator.flag(Flag::H));
    assert!(!emulator.flag(Flag::N));
    assert!(!emulator.flag(Flag::Z));
}

fn test_add_with_carry_flags(emulator: Emulator) {
    assert!(emulator.flag(Flag::CY));
    assert!(!emulator.flag(Flag::H));
    assert!(!emulator.flag(Flag::N));
    assert!(!emulator.flag(Flag::Z));
}

fn test_add_with_half_carry_flags(emulator: Emulator) {
    assert!(!emulator.flag(Flag::CY));
    assert!(emulator.flag(Flag::H));
    assert!(!emulator.flag(Flag::N));
    assert!(!emulator.flag(Flag::Z));
}

fn test_add_zeroed_results(emulator: Emulator) {
    assert_eq!(emulator.a(), 0x00);
    assert!(!emulator.flag(Flag::CY));
    assert!(!emulator.flag(Flag::H));
    assert!(!emulator.flag(Flag::N));
    assert!(emulator.flag(Flag::Z));
}

mod add_hl_location_to_a {
    use super::*;

    fn run(a: u8, b: u8) -> Emulator {
        let opcode = ADD_HL_LOCATION_TO_A.opcodes()[0];

        let mut emulator = common::setup_read_hl_location(opcode, b, false);

        emulator.set_a(a);

        emulator.process_opcode().unwrap();

        emulator
    }

    #[test]
    fn cycles() {
        let emulator = run(0x02, 0x01);

        assert_eq!(emulator.cycles(), 2);
    }

    #[test]
    fn simple () {
        let emulator = run(0x02, 0x01);

        assert_eq!(emulator.a(), 0x03);
        test_add_simple_flags(emulator);
    }

    #[test]
    fn with_carry() {
        let emulator: Emulator = run(0xf0, 0x11);

        assert_eq!(emulator.a(), 0x01);
        test_add_with_carry_flags(emulator);
    }

    #[test]
    fn with_half_carry() {
        let emulator: Emulator = run(0x0f, 0x01);

        assert_eq!(emulator.a(), 0x10);
        test_add_with_half_carry_flags(emulator);
    }

    #[test]
    fn zeroed() {
        let emulator: Emulator = run(0x00, 0x00);

        test_add_zeroed_results(emulator);
    }
}

mod add_immediate_e_to_sp {
    use super::*;

    fn run(sp: u16, e: i8) -> Emulator {
        let opcode = ADD_IMMEDIATE_E_TO_SP.opcodes()[0];

        let mut emulator = common::setup_read_immediate_e(opcode, e, false);

        emulator.set_stack_pointer(sp);

        emulator.process_opcode().unwrap();

        emulator
    }

    #[test]
    #[ignore = "issue with 16-bit arithmetic"]
    fn cycles() {
        let emulator = run(0, 1);

        assert_eq!(emulator.cycles(), 4);
    }
}

mod add_immediate_n_to_a {
    use super::*;

    fn run(a: u8, b: u8) -> Emulator {
        let opcode = ADD_IMMEDIATE_N_TO_A.opcodes()[0];

        let mut emulator = common::setup_read_immediate_n(opcode, b, false);

        emulator.set_a(a);

        emulator.process_opcode().unwrap();

        emulator
    }

    #[test]
    fn cycles() {
        let emulator = run(0x00, 0x01);

        assert_eq!(emulator.cycles(), 2);
    }

    #[test]
    fn simple () {
        test_add_simple(run);
    }

    #[test]
    fn with_carry() {
        test_add_with_carry(run);
    }

    #[test]
    fn with_half_carry() {
        test_add_with_half_carry(run);
    }

    #[test]
    fn zeroed() {
        test_add_zeroed(run);
    }
}

mod add_register_to_a {
    use super:: *;

    fn run(opcode: u8, a: u8, b: u8) -> Emulator {
        let register = opcode.parse_register(0b00_000_111).unwrap();

        let mut emulator = common::simple_emulator(opcode);

        emulator.set_a(a);
        emulator.set_register(register, b);

        emulator.process_opcode().unwrap();

        emulator
    }

    fn run_when_a(a: u8, b: u8) -> Emulator {

        let mut emulator = common::simple_emulator(opcode);

        emulator.set_a(a);
        emulator.set_register(register, b);

        emulator.process_opcode().unwrap();

        emulator
    }

    #[test]
    fn cycles() {
        for opcode in ADD_REGISTER_TO_A.opcodes() {
            let emulator = run(opcode, 0x00, 0x01);
    
            assert_eq!(emulator.cycles(), 1);
        }
    }

    #[test]
    fn simple () {
        for opcode in ADD_REGISTER_TO_A.opcodes() {
            let register = opcode.parse_register(0b00_000_111).unwrap();

            match register {
                Register::A => {
                    let emulator = run_when_a(0x01, 0x01);

                    assert_eq!(emulator.a(), 0x02);

                    test_add_simple_results(emulator, with_carry)
                },
                _ => test_add_simple(opcode, run),
            };

        }
    }

    #[test]
    fn with_carry() {
        for opcode in ADD_REGISTER_TO_A.opcodes() {
            test_add_with_carry(opcode, run);
        }
    }

    #[test]
    fn with_half_carry() {
        for opcode in ADD_REGISTER_TO_A.opcodes() {
            test_add_with_half_carry(opcode, run);
        }
    }

    #[test]
    fn zeroed() {
        for opcode in ADD_REGISTER_TO_A.opcodes() {
            test_add_zeroed(opcode, run);
        }
    }
}

mod add_with_carry_hl_location_to_a {
    use super::*;

    fn run(a: u8, b: u8, carry: bool) -> Emulator {
        let opcode = ADD_WITH_CARRY_HL_LOCATION_TO_A.opcodes()[0];

        let mut emulator = common::setup_read_hl_location(opcode, b, false);

        emulator.set_a(a);
        emulator.set_flag(Flag::CY, carry);

        emulator.process_opcode().unwrap();

        emulator
    }

    #[test]
    fn cycles() {
        let emulator = run(0x00, 0x01, true);

        assert_eq!(emulator.cycles(), 2);
    }

    #[test]
    fn simple () {
        test_adc_simple(run, );
    }

    #[test]
    fn with_carry() {
        test_adc_with_carry(run);
    }

    #[test]
    fn with_half_carry() {
        test_adc_with_half_carry(run);
    }

    #[test]
    fn zeroed() {
        test_adc_zeroed(run);
    }
}

mod add_with_carry_immediate_n_to_a {
    use super::*;

    fn run(a: u8, b: u8, carry: bool) -> Emulator {
        let opcode = ADD_WITH_CARRY_IMMEDIATE_N_TO_A.opcodes()[0];

        let mut emulator = common::setup_read_immediate_n(opcode, b, false);

        emulator.set_a(a);
        emulator.set_flag(Flag::CY, carry);

        emulator.process_opcode().unwrap();

        emulator
    }

    #[test]
    fn cycles() {
        let emulator = run(0x00, 0x01, true);

        assert_eq!(emulator.cycles(), 2);
    }

    #[test]
    fn simple () {
        test_adc_simple(run);
    }

    #[test]
    fn with_carry() {
        test_adc_with_carry(run);
    }

    #[test]
    fn with_half_carry() {
        test_adc_with_half_carry(run);
    }

    #[test]
    fn zeroed() {
        test_adc_zeroed(run);
    }
}