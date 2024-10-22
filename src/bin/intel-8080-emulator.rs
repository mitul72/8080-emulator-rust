extern crate sdl2;
use intel_8080_emu_rust::emulator::machine::SpaceInvadersMachine;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{event::Event, pixels::Color, rect::Rect};
use std::time::Duration;
use std::time::Instant;

const SCALE_FACTOR: u32 = 2;
const SCREEN_WIDTH: u32 = 224 * SCALE_FACTOR;
const SCREEN_HEIGHT: u32 = 256 * SCALE_FACTOR;
const VIDEO_MEM_START: usize = 0x2400;
const VIDEO_MEM_END: usize = 0x4000;

// TODO: Run with cargo clippy -- -W clippy::pedantic

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut invaders = SpaceInvadersMachine::new();

    let window = video_subsystem
        .window(
            "Intel 8080 Emulator",
            (SCREEN_WIDTH) as u32,
            (SCREEN_HEIGHT) as u32,
        )
        .position_centered()
        .build()
        .unwrap();
    let mut last_time = Instant::now();
    let mut frame_count = 0;
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let target_fps = 120;
    let frame_duration = Duration::from_millis(1000 / target_fps);

    'running: loop {
        // this starts emulation per 33000 cycles
        let frame_start = Instant::now();
        invaders.start_emulation();
        draw_screen(&mut canvas, &invaders.get_memory());
        frame_count += 1;
        if last_time.elapsed() >= Duration::new(1, 0) {
            println!("FPS: {}", frame_count);
            frame_count = 0;
            last_time = Instant::now();
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    invaders.handle_key_down(keycode);
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    invaders.handle_key_up(keycode);
                }
                _ => {}
            }
        }

        let frame_time = frame_start.elapsed();
        if frame_time < frame_duration {
            std::thread::sleep(frame_duration - frame_time);
        }
    }
}

fn draw_screen(canvas: &mut Canvas<Window>, memory: &[u8]) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    // Loop over each byte in video memory
    for addr in VIDEO_MEM_START..VIDEO_MEM_END {
        let byte = memory[addr];

        // Each byte represents 8 vertical pixels
        for bit in 0..8 {
            let pixel_on = (byte >> bit) & 1;

            if pixel_on != 0 {
                // Calculate the x and y coordinates (rotated for Space Invaders)
                let pixel_index = (addr - VIDEO_MEM_START) * 8 + bit;
                let x = (pixel_index % 256) as u32;
                let y = (pixel_index / 256) as u32;

                // Scale the coordinates to fit the doubled screen size
                let scaled_x = (256 - x).wrapping_mul(SCALE_FACTOR);
                let scaled_y = y.wrapping_mul(SCALE_FACTOR);

                // Draw a scaled rectangle to represent the pixel
                canvas.set_draw_color(Color::WHITE);
                let rect = Rect::new(
                    scaled_y as i32,
                    scaled_x as i32,
                    SCALE_FACTOR, // Width of the rectangle (scaled pixel)
                    SCALE_FACTOR, // Height of the rectangle (scaled pixel)
                );
                canvas.fill_rect(rect).unwrap();
            }
        }
    }

    canvas.present();
}
