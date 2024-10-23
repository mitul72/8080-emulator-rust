#![cfg(not(feature = "wasm"))]

use super::cpu::{self, CPU};

pub struct SpaceInvadersMachine {
    cpu: CPU,
    which_interrupt: u8,
}

impl SpaceInvadersMachine {
    pub fn new() -> Self {
        let mut machine = SpaceInvadersMachine {
            cpu: CPU::new(),
            which_interrupt: 1,
        };
        // Initialize ROM by loading files
        machine.read_multiple_files_into_memory(
            vec![
                "roms/space_invaders/invaders.h",
                "roms/space_invaders/invaders.g",
                "roms/space_invaders/invaders.f",
                "roms/space_invaders/invaders.e",
            ],
            0,
        );
        // machine.read_multiple_files_into_memory(
        //     vec![
        //         "roms/gunfight/7609h.bin",
        //         "roms/gunfight/7609g.bin",
        //         "roms/gunfight/7609f.bin",
        //         "roms/gunfight/7609e.bin",
        //     ],
        //     0,
        // );
        machine
    }

    fn read_multiple_files_into_memory(&mut self, filenames: Vec<&str>, offset: usize) {
        // Implement loading multiple binary files into memory starting at 'offset'
        let mut current_offset = offset;
        for filename in filenames {
            let rom = std::fs::read(filename).expect(&format!("Failed to load {}", filename));
            self.cpu.state.memory[current_offset..current_offset + rom.len()]
                .copy_from_slice(&rom[..]);
            current_offset += rom.len();
        }
    }

    pub fn get_framebuffer(&self) -> &[u8] {
        &self.cpu.state.memory[0x2400..]
    }

    pub fn get_memory(&self) -> &[u8] {
        &self.cpu.state.memory
    }

    pub fn do_cpu(&mut self) {
        let mut cycles = 0;
        let cycles_per_interrupt = 16_666;

        while cycles < cycles_per_interrupt {
            let op_cycles = cpu::emulate_8080_op(&mut self.cpu.state) as i32;
            cycles += op_cycles;
        }
        if self.cpu.state.int_enable {
            if self.which_interrupt == 1 {
                cpu::generate_interrupt(&mut self.cpu.state, 1);
                self.which_interrupt = 2;
            } else {
                cpu::generate_interrupt(&mut self.cpu.state, 2);
                self.which_interrupt = 1;
            }
        }
    }

    pub fn start_emulation(&mut self) {
        self.do_cpu();
    }

    pub fn handle_key_down(&mut self, key: sdl2::keyboard::Keycode) {
        self.cpu.handle_key_down(key);
    }

    pub fn handle_key_up(&mut self, key: sdl2::keyboard::Keycode) {
        self.cpu.handle_key_up(key);
    }
}
