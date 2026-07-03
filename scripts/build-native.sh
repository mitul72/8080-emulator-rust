#!/usr/bin/env bash
set -euo pipefail

# Build and run the native SDL2 Space Invaders emulator.
# Requires: SDL2 development libraries installed on your system.
#   macOS:  brew install sdl2
#   Debian/Ubuntu: sudo apt install libsdl2-dev
#   Arch:   sudo pacman -S sdl2

ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "Building native emulator..."
cargo build --release --bin i8080_emulator

echo "Done. Binary: $ROOT/target/release/i8080_emulator"
