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

    pub fn time_usec(&self) -> f64 {
        // Returns the elapsed time in microseconds since the program started
        self.last_timer.elapsed().as_secs_f64()
    }

    pub fn do_cpu(&mut self) {
        let now = self.time_usec();

        // Initialize the timer and interrupts on the first run
        if self.last_timer.elapsed().as_secs_f64() == 0.0 {
            self.last_timer = Instant::now();
            self.next_interrupt = self.last_timer.elapsed().as_secs_f64() * 1_000_000.0 + 16000.0;
            println!("next_int: {}", self.next_interrupt);
            self.which_interrupt = 1;
        }

        // Handle interrupts
        if self.cpu.state.int_enable && (now > self.next_interrupt) {
            if self.which_interrupt == 1 {
                cpu::generate_interrupt(&mut self.cpu.state, 1); // Generate INT 1
                self.which_interrupt = 2;
            } else {
                cpu::generate_interrupt(&mut self.cpu.state, 2); // Generate INT 2
                self.which_interrupt = 1;
            }
            self.next_interrupt = now + 8000.0; // Schedule next interrupt
        }

        // How much time has passed and how many CPU cycles are required to catch up?
        let since_last = now - self.last_timer.elapsed().as_secs_f64();
        println!("since_last: {}", since_last);
        let cycles_to_catch_up = (2.0 * since_last) as i32; // 2 MHz CPU
        while self.cpu.state.cycles < cycles_to_catch_up {
            self.cpu.state.cycles += cpu::emulate_8080_op(&mut self.cpu.state) as i32;
        }
        self.cpu.state.cycles = 0;

        // Update the last timer value to now
        self.last_timer = Instant::now();
    }

    pub fn start_emulation(&mut self) {
        // Create a timer with 1 millisecond intervals
        let interval = Duration::from_millis(1);
        // println!("Starting emulation");

        // Run the emulator loop
        let start = Instant::now();
        // Call the `do_cpu` method which simulates the CPU
        self.do_cpu();

        // Sleep for the remaining time in the interval to ensure it runs every 1 ms
        let elapsed = start.elapsed();
        if elapsed < interval {
            thread::sleep(interval - elapsed);
        }
    }

    pub fn key_down(&mut self, key: sdl2::keyboard::Keycode) {
        // match key {
        //     sdl2::keyboard::Keycode::Space => self.in_port1 |= 0x10,
        //     sdl2::keyboard::Keycode::C => self.in_port1 |= 0x04,
        //     sdl2::keyboard::Keycode::Num1 => self.in_port1 |= 0x01,
        //     _ => {}
        // }
    }

    pub fn key_up(&mut self, key: sdl2::keyboard::Keycode) {
        // match key {
        //     sdl2::keyboard::Keycode::Space => self.in_port1 &= !0x10,
        //     sdl2::keyboard::Keycode::C => self.in_port1 &= !0x04,
        //     sdl2::keyboard::Keycode::Num1 => self.in_port1 &= !0x01,
        //     _ => {}
        // }
    }
}
