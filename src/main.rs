extern crate sdl2;
use std::thread;

use intel_8080_emu_rust::emulator::{cpu, machine};

pub mod disassembler;
pub mod utils;

pub fn main() {
    let file_path = utils::get_file_path();
    let memory = utils::read_bin_file(&file_path);
    // disassembler::Disassembler::disassemble(&memory);
    let mut cpu = cpu::CPU::new();
    cpu.init_rom(memory);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Space Invaders", 224, 256)
        .position_centered()
        .build()
        .unwrap();

    cpu.run();

    let mut machine = machine::SpaceInvaderMachine::new(&mut cpu);
}
