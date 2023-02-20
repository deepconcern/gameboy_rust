extern crate bevy;
extern crate emulation;

use bevy::prelude::*;
use emulation::{Emulator, Register};

fn main() {
    App::new().add_system(hello_world_system).run();
}

fn hello_world_system() {
    let mut emulator = Emulator::new();

    println!("A: {}", emulator.get_register(&Register::A));
    println!("Add B to A");
    emulator.process_opcode(0b10000000);
    println!("A: {}", emulator.get_register(&Register::A));
}
