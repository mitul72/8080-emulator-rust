#![cfg(feature = "wasm")]

use crate::emulator::cpu;
use crate::emulator::cpu::CPU;
// use crate::emulator::machine;
// use crate::emulator::machine::SpaceInvadersMachine;
use std::time::Duration;
use std::time::Instant;
use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::{window, CanvasRenderingContext2d, Document, HtmlCanvasElement, ImageData};

const SCREEN_WIDTH: usize = 224;
const SCREEN_HEIGHT: usize = 256;
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
    context: CanvasRenderingContext2d,
    image_data: ImageData,
}

#[wasm_bindgen]
impl SpaceInvadersMachine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<SpaceInvadersMachine, JsValue> {
        // Initialize CPU and interrupts
        let cpu = CPU::new();
        let which_interrupt = 1;

        // Access the canvas and context
        let window = web_sys::window().ok_or("No global `window` exists")?;
        let document = window
            .document()
            .ok_or("Should have a document on window")?;
        let canvas = document
            .get_element_by_id("gameCanvas")
            .ok_or("Document should have a canvas element with id 'gameCanvas'")?
            .dyn_into::<HtmlCanvasElement>()?;
        let context = canvas
            .get_context("2d")?
            .ok_or("Canvas should have a 2d context")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        // Create an ImageData object
        let image_data = context.create_image_data_with_sw_and_sh(
            (SCREEN_WIDTH as f64).into(),
            (SCREEN_HEIGHT as f64).into(),
        )?;

        Ok(SpaceInvadersMachine {
            cpu,
            which_interrupt,
            context,
            image_data,
        })
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

    #[wasm_bindgen]
    pub fn get_cpu_state(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&serde_json::json!({
            "pc": self.cpu.state.pc,
            "sp": self.cpu.state.sp,
            "a": self.cpu.state.a,
            "b": self.cpu.state.b,
            "c": self.cpu.state.c,
            "d": self.cpu.state.d,
            "e": self.cpu.state.e,
            "h": self.cpu.state.h,
            "l": self.cpu.state.l,
            "flags": self.cpu.state.get_flags_as_byte(),
            "int_enable": self.cpu.state.int_enable
        }))
        .unwrap()
    }

    #[wasm_bindgen]
    pub fn get_last_instructions(&self) -> JsValue {
        let instructions = self.cpu.state.get_instructions_in_order();
        serde_wasm_bindgen::to_value(&instructions).unwrap()
    }

    // #[wasm_bindgen]
    // pub fn get_last_instructions(&self) -> js_sys::Array {}

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
        for _ in 0..4 {
            self.do_cpu();
        }
        self.draw_screen();
    }

    pub fn draw_screen(&mut self) {
        match self.get_frame_image_data() {
            Ok(image_data) => {
                if let Err(e) = self.context.put_image_data(&image_data, 0.0, 0.0) {
                    console::error_1(&format!("Failed to put image data: {:?}", e).into());
                }
            }
            Err(e) => {
                console::error_1(&format!("Failed to get frame image data: {:?}", e).into());
            }
        }
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

    pub fn get_frame_image_data(&self) -> Result<ImageData, JsValue> {
        let mut pixels = vec![0u8; SCREEN_WIDTH * SCREEN_HEIGHT * 4];

        for addr in VIDEO_MEM_START..VIDEO_MEM_END {
            let byte = self.cpu.state.memory[addr];
            if byte == 0 {
                continue;
            }

            // Each byte represents 8 vertical pixels
            for bit in 0..8 {
                if (byte >> bit) & 1 != 0 {
                    // Rotate the framebuffer 90° CCW into portrait orientation
                    let pixel_index = (addr - VIDEO_MEM_START) * 8 + bit;
                    let x = pixel_index % SCREEN_HEIGHT;
                    let y = pixel_index / SCREEN_HEIGHT;

                    let idx = ((SCREEN_HEIGHT - 1 - x) * SCREEN_WIDTH + y) * 4;
                    pixels[idx..idx + 4].copy_from_slice(&[255, 255, 255, 255]);
                }
            }
        }

        ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(&pixels),
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )
    }

    #[wasm_bindgen]
    pub fn handle_key_down(&mut self, key: u8) {
        self.cpu.handle_key_down(key);
    }

    #[wasm_bindgen]
    pub fn handle_key_up(&mut self, key: u8) {
        self.cpu.handle_key_up(key);
    }
}
