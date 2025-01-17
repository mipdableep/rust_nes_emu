pub mod colors_palette;
pub mod frame;
mod render_nes;
#[cfg(test)]
mod test_frame_rendering;
mod user_input;

use crate::bus::Bus;
use crate::ppu::frame::screen_rendering_constants::SCREEN_WIDTH;
use crate::ppu::frame::Frame;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::EventPump;

const SCANLINE_LENGTH_PIXELS: usize = 341;
const SCANLINES_PER_FRAME: usize = 262;
const NMI_SCANLINE: usize = 241;

pub struct PPU<'a> {
    ppu_cycles_in_current_scanline: usize, // the scanline lasts for 341 ppu cycles
    scanlines_in_current_frame: usize,     // each frame has 262 scanlines, with NMI in scanline 240
    cur_scanline_x_offset: usize, // we will compute the offset for the left pixel in the current scanline once per scanline
    cur_scanline_y_offset: usize,
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
        self.handle_background_one_cycle(frame);
        self.ppu_cycles_in_current_scanline += 1;
        // todo - actually draw something
        self.trigger_new_scanline_if_needed();

        if self.scanlines_in_current_frame >= SCANLINES_PER_FRAME {
            self.scanlines_in_current_frame -= SCANLINES_PER_FRAME;
            self.copy_sprite_to_frame(frame);
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
