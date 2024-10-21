extern crate sdl2;
use super::cpu::{self, CPU};
use heapless::spsc::{Consumer, Producer};
use std::thread::{self};
use std::time::{Duration, Instant};

pub struct SpaceInvadersMachine {
    cpu: CPU,
    last_timer: Instant,
    next_interrupt: f64,
    which_interrupt: u8,
    paused: bool,
}

impl SpaceInvadersMachine {
    pub fn new() -> Self {
        let mut machine = SpaceInvadersMachine {
            cpu: CPU::new(),
            last_timer: Instant::now(),
            next_interrupt: 16000.0,
            which_interrupt: 1,
            paused: false,
        };
        // Initialize ROM by loading files
        machine.read_file_into_memory("roms/space_invaders/invaders", 0);
        machine
    }

    fn read_file_into_memory(&mut self, filename: &str, offset: usize) {
        // Implement loading binary file data into memory starting at 'offset'
        let rom = std::fs::read(filename).expect(&format!("Failed to load {}", filename));
        self.cpu.state.memory[offset..offset + rom.len()].copy_from_slice(&rom[..]);
    }

    pub fn get_framebuffer(&self) -> &[u8] {
        &self.cpu.state.memory[0x2400..]
    }

    pub fn get_memory(&self) -> &[u8] {
        &self.cpu.state.memory
    }

    pub fn do_cpu(&mut self) {
        let mut cycles = 0;
        let cycles_per_interrupt = 16_666; // Approximate value
        let time = Instant::now();
        while cycles < cycles_per_interrupt {
            let op_cycles = cpu::emulate_8080_op(&mut self.cpu.state) as i32;
            cycles += op_cycles;
        }
        print!("time elapsed for 16_666: {} ", time.elapsed().as_secs_f64());
        // Generate the interrupt
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
        // Create a timer with 1 millisecond intervals
        // let interval = Duration::from_millis(1);
        // println!("Starting emulation");

        // Run the emulator loop
        // let start = Instant::now();
        // Call the `do_cpu` method which simulates the CPU
        self.do_cpu();

        // Sleep for the remaining time in the interval to ensure it runs every 1 ms
        // let elapsed = start.elapsed();
        // if elapsed < interval {
        //     // thread::sleep(interval - elapsed);
        // }
    }
    pub fn handle_key_down(&mut self, key: sdl2::keyboard::Keycode) {
        self.cpu.handle_key_down(key);
    }
    pub fn handle_key_up(&mut self, key: sdl2::keyboard::Keycode) {
        self.cpu.handle_key_up(key);
    }
}
