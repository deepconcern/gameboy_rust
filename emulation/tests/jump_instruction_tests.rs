mod common;

use emulation::{
    addresses::PROGRAM_COUNTER_START,
    flag::Flag,
    instruction::jump_instructions::{
        JUMP_TO_HL,
        JUMP_TO_IMMEDIATE_E,
        JUMP_TO_IMMEDIATE_E_IF_CONDITION,
        JUMP_TO_IMMEDIATE_NN,
        JUMP_TO_IMMEDIATE_NN_IF_CONDITION,
    },
    opcode::Opcode,
    Emulator,
};

mod jump_to_hl {
    use super::*;
    fn run() -> Emulator {
        let mut emulator = common::simple_emulator(JUMP_TO_HL.opcodes()[0]);

        emulator.set_hl(0x3333);

        emulator.process_opcode().unwrap();

        emulator
    }

    #[test]
    #[ignore = "need to think about cycle implementation"]
    fn cycles() {
        let emulator = run();

        assert_eq!(emulator.cycles(), 1);
    }

    #[test]
    fn simple() {
        let emulator = run();

        assert_eq!(emulator.program_counter(), 0x3333u16);
    }
}

mod jump_to_immediate_e {
    use super::*;

    fn run(value: i8) -> Emulator {
        let opcode = JUMP_TO_IMMEDIATE_E.opcodes()[0];
        
        let mut emulator = common::setup_read_immediate_e(opcode, value, false);
        
        emulator.process_opcode().unwrap();

        emulator
    }

    #[test]
    #[ignore = "need to think about cycle implementation"]
    fn cycles() {
        let emulator = run(3);

        assert_eq!(emulator.cycles(), 3);
    }

    #[test]
    fn negative() {
        let emulator = run(-3);

        assert_eq!(emulator.program_counter(), PROGRAM_COUNTER_START.wrapping_sub(3) + 2);
    }

    #[test]
    fn positive() {
        let emulator = run(3);

        assert_eq!(emulator.program_counter(), PROGRAM_COUNTER_START + 5);
    }
}

mod jump_to_immediate_e_if_condition {
    use super::*;

    fn run(opcode: u8, e: i8, carry: bool, zero: bool) -> Emulator {
        let mut emulator = common::setup_read_immediate_e(opcode, e, false);

        emulator.set_flag(Flag::CY, carry);
        emulator.set_flag(Flag::Z, zero);

        emulator.process_opcode().unwrap();

        emulator
    }

    #[test]
    #[ignore = "need to think about cycle implementation"]
    fn cycles() {
        for (carry, zero) in [(true, true), (false, false)] {
            for opcode in JUMP_TO_IMMEDIATE_E_IF_CONDITION.opcodes() {
                let condition = opcode.parse_condition(0b00_011_000).unwrap();
                let emulator = run(opcode, 5, carry, zero);

                if condition.check(&emulator) {
                    assert_eq!(emulator.cycles(), 4);
                } else {
                    assert_eq!(emulator.cycles(), 3);
                }
            }
        }
    }

    #[test]
    fn negative() {
        for (carry, zero) in [(true, true), (false, false)] {
            for opcode in JUMP_TO_IMMEDIATE_E_IF_CONDITION.opcodes() {
                let condition = opcode.parse_condition(0b00_011_000).unwrap();
                let emulator = run(opcode, -5, carry, zero);

                if condition.check(&emulator) {
                    assert_eq!(emulator.program_counter(), PROGRAM_COUNTER_START.wrapping_sub(3));
                } else {
                    assert_eq!(emulator.program_counter(), PROGRAM_COUNTER_START + 2);
                }
            }
        }
    }

    #[test]
    fn positive() {

        for (carry, zero) in [(true, true), (false, false)] {
            for opcode in JUMP_TO_IMMEDIATE_E_IF_CONDITION.opcodes() {
                let condition = opcode.parse_condition(0b00_011_000).unwrap();
                let emulator = run(opcode, 5, carry, zero);

                if condition.check(&emulator) {
                    assert_eq!(emulator.program_counter(), PROGRAM_COUNTER_START + 7);
                } else {
                    assert_eq!(emulator.program_counter(), PROGRAM_COUNTER_START + 2);
                }
            }
        }
    }
}

mod jump_to_immediate_nn {
    use super::*;

    fn run() -> Emulator {
        let opcode = JUMP_TO_IMMEDIATE_NN.opcodes()[0];
        
        let mut emulator = common::setup_read_immediate_nn(opcode, 0x3333, false);

        emulator.process_opcode().unwrap();

        emulator
    }

    #[test]
    #[ignore = "need to think about cycle implementation"]
    fn cycles() {
        let emulator = run();

        assert_eq!(emulator.cycles(), 4);
    }

    #[test]
    fn simple() {
        let emulator = run();

        assert_eq!(emulator.program_counter(), 0x3333u16);
    }
}

mod jump_to_immediate_nn_if_condition {
    use super::*;

    fn run(opcode: u8, carry: bool, zero: bool) -> Emulator {
        let mut emulator = common::setup_read_immediate_nn(opcode, 0x3333, false);

        emulator.set_flag(Flag::CY, carry);
        emulator.set_flag(Flag::Z, zero);

        emulator.process_opcode().unwrap();

        emulator
    }

    #[test]
    #[ignore = "need to think about cycle implementation"]
    fn cycles() {
        for (carry, zero) in [(true, true), (false, false)] {
            for opcode in JUMP_TO_IMMEDIATE_NN_IF_CONDITION.opcodes() {
                let condition = opcode.parse_condition(0b00_011_000).unwrap();
                let emulator = run(opcode, carry, zero);

                if condition.check(&emulator) {
                    assert_eq!(emulator.cycles(), 4);
                } else {
                    assert_eq!(emulator.cycles(), 3);
                }
            }
        }
    }

    #[test]
    fn simple() {
        for (carry, zero) in [(true, true), (false, false)] {
            for opcode in JUMP_TO_IMMEDIATE_NN_IF_CONDITION.opcodes() {
                let condition = opcode.parse_condition(0b00_011_000).unwrap();
                let emulator = run(opcode, carry, zero);

                if condition.check(&emulator) {
                    assert_eq!(emulator.program_counter(), 0x3333);
                } else {
                    assert_eq!(emulator.program_counter(), PROGRAM_COUNTER_START + 3);
                }
            }
        }
    }
}