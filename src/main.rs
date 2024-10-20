extern crate sdl2;
use intel_8080_emu_rust::emulator::machine::SpaceInvadersMachine;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{event::Event, pixels::Color, rect::Rect};
use std::time::Duration;
use std::time::Instant;

pub mod disassembler;
pub mod utils;

const SCREEN_WIDTH: u32 = 224 * 2;
const SCREEN_HEIGHT: u32 = 256 * 2;
// const PIXEL_SIZE: u32 = 2;

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

    // let producer_thread = thread::spawn(move || loop {
    //     invaders.start_emulation();
    //     // let framebuffer = invaders.get_framebuffer();
    //     // producer.enqueue(framebuffer).unwrap();
    // });

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
                    // this will generate an interrupt
                    // generate_interrupt(&mut invaders, 1);
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

        // // 16ms delay to achieve 60fps
        // std::thread::sleep(Duration::from_millis(1));
    }

    // let (tx, rx) = channel();

    // let emulation_thread = thread::spawn(move || {
    //     invaders.start_emulation();
    //     loop {
    //         // Emulate one frame
    //         invaders.emulate_frame();

    //         // Send the framebuffer to the main thread
    //         let framebuffer = invaders.get_framebuffer();
    //         if tx.send(framebuffer).is_err() {
    //             break;
    //         }

    //         // Sleep to simulate the frame rate
    //         thread::sleep(Duration::from_millis(16));
    //     }
    // });

    // rx.recv().unwrap();

    // Game loop
}

fn draw_screen(canvas: &mut Canvas<Window>, memory: &[u8]) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    // Video memory starts at 0x2400
    let video_mem_start = 0x2400;
    let video_mem_end = 0x4000; // 0x3FFF + 1

    // Scaling factor for the screen (since you doubled the dimensions)
    let scale_factor: u32 = 2; // You doubled both width and height, so scale factor is 2

    // Loop over each byte in video memory
    for addr in video_mem_start..video_mem_end {
        let byte = memory[addr];

        // Each byte represents 8 vertical pixels
        for bit in 0..8 {
            let pixel_on = (byte >> bit) & 1;

            if pixel_on != 0 {
                // Calculate the x and y coordinates (rotated for Space Invaders)
                let pixel_index = (addr - video_mem_start) * 8 + bit;
                let x = (pixel_index % 256) as u32;
                let y = (pixel_index / 256) as u32;

                // Scale the coordinates to fit the doubled screen size
                let scaled_x = (224u32.wrapping_sub(x)).wrapping_mul(scale_factor);
                let scaled_y = y.wrapping_mul(scale_factor);

                // Draw a scaled rectangle to represent the pixel
                canvas.set_draw_color(Color::WHITE);
                let rect = Rect::new(
                    scaled_y as i32,
                    scaled_x as i32,
                    scale_factor, // Width of the rectangle (scaled pixel)
                    scale_factor, // Height of the rectangle (scaled pixel)
                );
                canvas.fill_rect(rect).unwrap();
            }
        }
    }

    // Update the canvas
    canvas.present();
}
