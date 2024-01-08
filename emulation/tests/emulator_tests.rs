mod common;

use std::collections::HashSet;

use emulation::instruction::general_instructions::UNIMPLEMENTED_OPCODES;

#[test]
fn test_instructions() {
    let emulator = common::simple_emulator(0x00);

    let mut matched = HashSet::<(bool, u8)>::new();
    let mut unmatched = HashSet::<(bool, u8)>::new();

    for opcode in UNIMPLEMENTED_OPCODES {
        matched.insert((false, opcode));
    }

    for prefix_required in [true, false] {
        for i in 0u8..=u8::MAX {
            if matched.contains(&(prefix_required, i)) {
                continue;
            };

            match emulator.instruction_name((prefix_required, i)) {
                Some(_) => matched.insert((prefix_required, i)),
                None => unmatched.insert((prefix_required, i)),
            };
        }
    }

    assert_eq!(unmatched.len(), 0usize, "Unmatched: {}", unmatched.iter().map(|(p, o)| {
        let ps = if *p {
            "prefix required"
        } else {
            "no prefix required"
        };

        format!("{:#04x} ({})", o, ps)
    }).collect::<Vec<String>>().join(", "));

    assert_eq!(matched.len(), 512usize);
}