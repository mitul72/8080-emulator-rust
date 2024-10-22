pub mod disassembler;
pub mod emulator;
pub mod space_invaders_wasm;
pub mod utils;

#[cfg(not(target_arch = "wasm32"))]
mod tests;
