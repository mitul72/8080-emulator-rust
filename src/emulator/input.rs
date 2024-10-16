use super::data_types;

fn handle_key_down(state: &mut data_types::State8080, key: sdl2::keyboard::Keycode) {
    match key {
        sdl2::keyboard::Keycode::Left => {
            state.in_port1 |= 0x20; // Set bit 5 (Player 1 Left)
        }
        sdl2::keyboard::Keycode::Right => {
            state.in_port1 |= 0x40; // Set bit 6 (Player 1 Right)
        }
        sdl2::keyboard::Keycode::Space => {
            state.in_port1 |= 0x10; // Set bit 4 (Player 1 Fire)
        }
        // Add other keys like Coin, Start, etc.
        _ => {}
    }
}

fn handle_key_up(state: &mut data_types::State8080, key: sdl2::keyboard::Keycode) {
    match key {
        sdl2::keyboard::Keycode::Left => {
            state.in_port1 &= !0x20; // Clear bit 5 (Player 1 Left)
        }
        sdl2::keyboard::Keycode::Right => {
            state.in_port1 &= !0x40; // Clear bit 6 (Player 1 Right)
        }
        sdl2::keyboard::Keycode::Space => {
            state.in_port1 &= !0x10; // Clear bit 4 (Player 1 Fire)
        }
        // Add other keys like Coin, Start, etc.
        _ => {}
    }
}
