use crate::emulator::cpu::{emulate_8080_op, CPU};
use std::fs::File;
use std::io::Read; // Adjust the path if needed

fn load_cpudiag(cpu: &mut CPU, filename: &str) {
    let mut file = File::open(filename).expect("Cannot open cpudiag.bin");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Error reading file");
    cpu.init_rom(buffer);
}

#[test]
fn test_cpudiag() {
    let mut cpu = CPU::new();

    // Load the cpudiag ROM
    load_cpudiag(&mut cpu, "roms/cpu_diag/cpudiag.bin");

    // Set the stack pointer and skip the first instruction
    cpu.state.sp = 0x2400;
    cpu.state.pc = 0x100; // Start after the OUT instruction

    let mut cycles = 0;
    let max_cycles = 10_000_000; // Arbitrary large cycle limit to avoid infinite loops

    while cpu.state.pc != 0x06E0 && cycles < max_cycles {
        emulate_8080_op(&mut cpu.state);
        cycles += 1;
    }

    // Check if the emulator reached the success loop (PC at 0x06E0)
    assert_eq!(
        cpu.state.pc, 0x06E0,
        "cpudiag did not reach the success loop"
    );

    // Optionally, you could also check the number of cycles
    println!("cpudiag test passed in {} cycles", cycles);
}
