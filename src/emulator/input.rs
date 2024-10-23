use super::cpu::CPU;

impl CPU {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn handle_key_down(&mut self, key: sdl2::keyboard::Keycode) {
        match key {
            sdl2::keyboard::Keycode::Left => {
                self.state.in_port1 |= 0x20; // Set bit 5 (Player 1 Left)
            }
            sdl2::keyboard::Keycode::Right => {
                self.state.in_port1 |= 0x40; // Set bit 6 (Player 1 Right)
            }
            sdl2::keyboard::Keycode::Space => {
                self.state.in_port1 |= 0x10; // Set bit 4 (Player 1 Fire)
            }
            sdl2::keyboard::Keycode::Tab => {
                self.state.in_port1 |= 0x01; // Set bit 0 (Coin)
            }
            sdl2::keyboard::Keycode::RETURN => {
                self.state.in_port1 |= 0x04; // Set bit 2 (Player 1 Start)
            }
            _ => {}
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn handle_key_up(&mut self, key: sdl2::keyboard::Keycode) {
        match key {
            sdl2::keyboard::Keycode::Left => {
                self.state.in_port1 &= !0x20; // Clear bit 5 (Player 1 Left)
            }
            sdl2::keyboard::Keycode::Right => {
                self.state.in_port1 &= !0x40; // Clear bit 6 (Player 1 Right)
            }
            sdl2::keyboard::Keycode::Space => {
                self.state.in_port1 &= !0x10; // Clear bit 4 (Player 1 Fire)
            }
            sdl2::keyboard::Keycode::Tab => {
                self.state.in_port1 &= !0x01; // Clear bit 0 (Coin)
            }
            sdl2::keyboard::Keycode::RETURN => {
                self.state.in_port1 &= !0x04; // Clear bit 2 (Player 1 Start)
            }
            _ => {}
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn handle_key_down(&mut self, key: u8) {
        self.state.in_port1 |= key;
    }

    #[cfg(target_arch = "wasm32")]
    pub fn handle_key_up(&mut self, key: u8) {
        self.state.in_port1 &= !key;
    }
}
