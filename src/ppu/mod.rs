pub mod colors_palette;
mod full_screen_rendering;
pub mod render_sdl;

use crate::bus::Bus;
use crate::ppu::render_sdl::Frame;
use sdl2::render::{Texture, WindowCanvas};

const SCANLINE_LENGTH_PIXELS: usize = 341;
const SCANLINES_PER_FRAME: usize = 262;
const NMI_SCANLINE: usize = 241;

pub struct PPU<'a> {
    ppu_cycles_in_current_scanline: usize, // the scanline lasts for 341 ppu cycles
    scanlines_in_current_frame: usize,     // each frame has 262 scanlines, with NMI in scanline 240
    pub palette_table: [u8; 32],
    pub bus: Option<&'a mut Bus>,
}

impl<'a> PPU<'a> {
    pub fn new(bus: &'a mut Bus) -> Self {
        PPU {
            ppu_cycles_in_current_scanline: 0,
            scanlines_in_current_frame: 0,
            palette_table: [0; 32],
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
    ) {
        self.ppu_cycles_in_current_scanline += 1;
        // todo - actually draw something
        self.trigger_new_scanline_if_needed();

        if self.scanlines_in_current_frame == NMI_SCANLINE
            && self.ppu_cycles_in_current_scanline == 0
        {
            self.render_full_screen_background(texture, frame, canvas);
            self.bus
                .as_mut()
                .unwrap()
                .ppu_registers
                .status_register
                .set_vblank_status(true);
            if self
                .bus
                .as_mut()
                .unwrap()
                .ppu_registers
                .control_register
                .get_vblank_nmi()
            {
                self.bus.as_mut().unwrap().nmi_generated = true;
            }
        }

        if self.scanlines_in_current_frame >= SCANLINES_PER_FRAME {
            self.scanlines_in_current_frame -= SCANLINES_PER_FRAME;
            self.bus
                .as_mut()
                .unwrap()
                .ppu_registers
                .status_register
                .set_vblank_status(false);
            // todo - disable nmi
        }
    }
}
