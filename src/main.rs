use intel_8080_emu_rust::emulator::cpu;

pub mod disassembler;
pub mod utils;

pub fn main() {
    let file_path = utils::get_file_path();
    let memory = utils::read_bin_file(&file_path);
    // disassembler::Disassembler::disassemble(&memory);
    let mut cpu = cpu::CPU::new();
    cpu.run(memory);
}
