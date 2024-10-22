#![cfg(feature = "wasm")]
use crate::emulator::cpu;
use crate::emulator::cpu::CPU;
// use crate::emulator::machine;
// use crate::emulator::machine::SpaceInvadersMachine;
use std::time::Duration;
use std::time::Instant;
use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, Document, HtmlCanvasElement};

const SCALE_FACTOR: u32 = 2;
const SCREEN_WIDTH: u32 = 224 * SCALE_FACTOR;
const SCREEN_HEIGHT: u32 = 256 * SCALE_FACTOR;
const VIDEO_MEM_START: usize = 0x2400;
const VIDEO_MEM_END: usize = 0x4000;
static mut CONTEXT: Option<CanvasRenderingContext2d> = None;
static mut CANVAS: Option<HtmlCanvasElement> = None;

// TODO: Run with cargo clippy -- -W clippy::pedantic

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn init() {
    let window = web_sys::window().expect("No global `window` exists");
    let document: Document = window.document().expect("Should have a document on window");
    let canvas = document
        .get_element_by_id("gameCanvas")
        .expect("Document should have a canvas element with id 'gameCanvas'")
        .dyn_into::<HtmlCanvasElement>()
        .expect("Canvas element should be of type HtmlCanvasElement");
    let context = canvas
        .get_context("2d")
        .expect("Canvas should have a 2d context")
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("Context should be a CanvasRenderingContext2d");
    unsafe {
        CONTEXT = Some(context);
        CANVAS = Some(canvas);
    }

    // let mut invaders: SpaceInvadersMachine = SpaceInvadersMachine::new();

    // let target_fps = 120;
    // let frame_duration = Duration::from_millis(1000 / target_fps);

    // loop {
    // this starts emulation per 33000 cycles
    // let frame_start = Instant::now();
    // invaders.start_emulation();
    // draw_screen(&invaders.get_memory());
    // let frame_time = frame_start.elapsed();
    // if frame_time < frame_duration {
    // std::thread::sleep(frame_duration - frame_time);
    // }
    // }
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn draw_screen(ptr: *const u8) {
    let context = unsafe { CONTEXT.as_ref().expect("Context should be initialized") };
    let canvas = unsafe { CANVAS.as_ref().expect("Canvas should be initialized") };
    // Clear the canvas
    context.set_fill_style(&JsValue::from_str("black"));
    context.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

    // Drawing logic
    const SCALE_FACTOR: f64 = 2.0;
    const VIDEO_MEM_START: usize = 0x2400;
    const VIDEO_MEM_END: usize = 0x4000;

    for addr in VIDEO_MEM_START..VIDEO_MEM_END {
        let mut byte = 0;
        unsafe {
            byte = *ptr.add(addr);
        }
        let base_pixel_index = (addr - VIDEO_MEM_START) * 8;

        for bit in 0..8 {
            let pixel_on = (byte >> bit) & 1;

            if pixel_on != 0 {
                let pixel_index = base_pixel_index + bit;
                let x = (pixel_index % 256) as f64;
                let y = (pixel_index / 256) as f64;

                let scaled_x = (256.0 - x) * (SCALE_FACTOR);
                let scaled_y = y * (SCALE_FACTOR);

                context.set_fill_style(&JsValue::from_str("white"));
                context.fill_rect(scaled_y, scaled_x, SCALE_FACTOR, SCALE_FACTOR);
            }
        }
    }
}

#[wasm_bindgen]
pub struct SpaceInvadersMachine {
    cpu: CPU,
    which_interrupt: u8,
}

#[wasm_bindgen]
impl SpaceInvadersMachine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SpaceInvadersMachine {
        SpaceInvadersMachine {
            cpu: CPU::new(),
            which_interrupt: 1,
        }
    }

    #[wasm_bindgen]
    pub fn load_rom(&mut self, rom_data: &[u8], offset: usize) {
        let end = offset + rom_data.len();
        self.cpu.state.memory[offset..end].copy_from_slice(rom_data);
    }

    #[wasm_bindgen]
    pub fn get_memory(&self) -> *const u8 {
        self.cpu.state.memory.as_ptr()
    }

    // Return the pointer to the framebuffer and its length
    #[wasm_bindgen]
    pub fn get_framebuffer_ptr(&self) -> *const u8 {
        self.cpu.state.memory[0x2400..].as_ptr()
    }

    #[wasm_bindgen]
    pub fn get_framebuffer_len(&self) -> usize {
        0x4000 - 0x2400
    }

    #[wasm_bindgen]
    pub fn start_emulation(&mut self) {
        self.do_cpu();
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
}
