use super::render_nes::ppu_render_constants::*;
use sdl2::render::{Texture, WindowCanvas};

#[derive(Debug, Eq, PartialEq)]
pub struct Frame {
    pub screen_state: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
}

impl Frame {
    pub fn new() -> Self {
        Self {
            screen_state: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, (r, g, b): (u8, u8, u8)) {
        if x >= SCREEN_WIDTH {
            return;
            // panic!("Trying to set pixel {x}, which is more than width {SCREEN_WIDTH}")
        }
        if y >= SCREEN_HEIGHT {
            return;
            // panic!("Trying to set pixel {y}, which is more than width {SCREEN_HEIGHT}")
        }
        self.screen_state[3 * (x + y * SCREEN_WIDTH)] = r;
        self.screen_state[3 * (x + y * SCREEN_WIDTH) + 1] = g;
        self.screen_state[3 * (x + y * SCREEN_WIDTH) + 2] = b;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> (u8, u8, u8) {
        (
            self.screen_state[3 * (x + y * SCREEN_WIDTH)],
            self.screen_state[3 * (x + y * SCREEN_WIDTH) + 1],
            self.screen_state[3 * (x + y * SCREEN_WIDTH) + 2],
        )
    }
}

pub fn update_texture_from_frame(texture: &mut Texture, frame: &Frame, canvas: &mut WindowCanvas) {
    texture
        .update(None, &frame.screen_state, SCREEN_WIDTH * 3)
        .unwrap();
    canvas.copy(texture, None, None).unwrap();
}
