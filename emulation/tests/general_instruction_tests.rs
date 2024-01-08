mod common;

use emulation::{
    instruction::general_instructions::PREFIX,
    Emulator,
};

mod prefix {
    use super::*;
    
    fn run() -> Emulator {
        let opcode = PREFIX.opcodes()[0];
        
        let mut emulator = common::simple_emulator(opcode);

        emulator.process_opcode().unwrap();

        emulator
    }

    #[test]
    fn cycles() {
        let emulator = run();

        assert_eq!(emulator.cycles(), 1);
    }

    #[test]
    fn simple() {
        let emulator = run();

        assert!(emulator.prefixed());
    }
}