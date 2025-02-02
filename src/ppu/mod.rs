use render_nes::ppu_render_constants::*;
pub use render_nes::ppu_render_constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub mod colors_palette;
pub mod frame;
mod render_nes;
#[cfg(test)]
mod test_frame_rendering;
mod user_input;

use crate::bus::Bus;
use crate::ppu::frame::Frame;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::EventPump;

const MAX_SPRITES_PER_LINE: usize = 8;

#[derive(Clone, Debug, Copy)]
pub struct SpritePixel {
    color: (u8, u8, u8),
    is_background: bool,
    sprite_index: u8,
}

pub struct PPU<'a> {
    ppu_cycles_in_current_scanline: usize, // the scanline lasts for 341 ppu cycles
    scanlines_in_current_frame: usize,     // each frame has 262 scanlines, with NMI in scanline 240
    cur_scanline_x_offset: usize, // we will compute the offset for the left pixel in the current scanline once per scanline
    cur_scanline_y_offset: usize,
    secondary_oam: [u8; 4 * MAX_SPRITES_PER_LINE],
    sprites_original_numbers: [u8; MAX_SPRITES_PER_LINE], // to remember where each sprite in the secondary oam came from
    next_line_sprite_pixels: [Option<SpritePixel>; SCREEN_WIDTH],
    number_of_sprites_in_scanline: usize,
    pub bus: Option<&'a mut Bus>,
}

#[macro_export]
macro_rules! ppu_mem {
    ($ppu: ident) => {
        bus!($ppu).ppu_memory
    };
}

#[macro_export]
macro_rules! palette {
    ($ppu: ident) => {
        bus!($ppu).ppu_memory.palette_table
    };
}

impl<'a> PPU<'a> {
    pub fn new(bus: &'a mut Bus) -> Self {
        PPU {
            ppu_cycles_in_current_scanline: 0,
            scanlines_in_current_frame: 0,
            cur_scanline_x_offset: 0,
            cur_scanline_y_offset: 0,
            secondary_oam: [0; 4 * MAX_SPRITES_PER_LINE],
            sprites_original_numbers: [0; MAX_SPRITES_PER_LINE],
            next_line_sprite_pixels: [None; SCREEN_WIDTH],
            number_of_sprites_in_scanline: 0,
            bus: Some(bus),
        }
    }

    fn trigger_new_scanline_if_needed(&mut self) {
        if self.ppu_cycles_in_current_scanline >= SCANLINE_LENGTH_PIXELS {
            self.ppu_cycles_in_current_scanline -= SCANLINE_LENGTH_PIXELS;
            self.scanlines_in_current_frame += 1;
        }
    }

    pub fn run_one_ppu_cycle(
        &mut self,
        texture: &mut Texture,
        frame: &mut Frame,
        canvas: &mut WindowCanvas,
        event_pump: &mut EventPump,
    ) {
        self.handle_sprites_one_cycle();
        self.handle_background_one_cycle(frame);
        self.ppu_cycles_in_current_scanline += 1;
        // todo - actually draw something
        self.trigger_new_scanline_if_needed();

        if self.scanlines_in_current_frame >= SCANLINES_PER_FRAME {
            self.scanlines_in_current_frame -= SCANLINES_PER_FRAME;
            update_texture_from_frame(texture, frame, canvas);
            canvas.present();
            self.handle_user_input(event_pump);
        }
    }
}

fn update_texture_from_frame(texture: &mut Texture, frame: &Frame, canvas: &mut WindowCanvas) {
    texture
        .update(None, &frame.screen_state, SCREEN_WIDTH * 3)
        .unwrap();
    canvas.copy(texture, None, None).unwrap();
}
