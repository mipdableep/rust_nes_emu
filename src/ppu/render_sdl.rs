use crate::ppu::colors_palette::SYSTEM_PALETTE;
use screen_rendering_constants::*;
use sdl2::render::{Texture, WindowCanvas};

pub mod screen_rendering_constants {
    pub const SCREEN_WIDTH: usize = 256;
    pub const SCREEN_HEIGHT: usize = 240;
    pub const SCREEN_FACTOR: usize = 2;
}

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

    pub fn draw_tile_one_frame(
        &mut self,
        tile_x: usize,
        tile_y: usize,
        tile: &[u8],
        tile_palette: [u8; 4],
    ) {
        for y in 0..=7 {
            let mut upper = tile[y];
            let mut lower = tile[y + 8];

            for x in (0..=7).rev() {
                let value = (1 & lower) << 1 | (1 & upper);
                upper = upper >> 1;
                lower = lower >> 1;
                let rgb = SYSTEM_PALETTE[tile_palette[value as usize] as usize];
                self.set_pixel(tile_x * 8 + x, tile_y * 8 + y, rgb)
            }
        }
    }
}

pub fn update_texture_from_frame(texture: &mut Texture, frame: &Frame, canvas: &mut WindowCanvas) {
    texture
        .update(None, &frame.screen_state, SCREEN_WIDTH * 3)
        .unwrap();
    canvas.copy(texture, None, None).unwrap();
}
