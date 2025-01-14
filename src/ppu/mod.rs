pub mod colors_palette;
mod full_screen_rendering;
pub mod render_sdl;
#[cfg(test)]
mod test_frame_rendering;
mod user_input;

use crate::bus::Bus;
use crate::ppu::render_sdl::Frame;
use crate::{bus, bus_mut};
use sdl2::render::{Texture, WindowCanvas};
use sdl2::EventPump;

const SCANLINE_LENGTH_PIXELS: usize = 341;
const SCANLINES_PER_FRAME: usize = 262;
const NMI_SCANLINE: usize = 241;

pub struct PPU<'a> {
    ppu_cycles_in_current_scanline: usize, // the scanline lasts for 341 ppu cycles
    scanlines_in_current_frame: usize,     // each frame has 262 scanlines, with NMI in scanline 240
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

macro_rules! status_reg {
    ($ppu: ident) => {
        bus_mut!($ppu).ppu_registers.status_register
    };
}

impl<'a> PPU<'a> {
    pub fn new(bus: &'a mut Bus) -> Self {
        PPU {
            ppu_cycles_in_current_scanline: 0,
            scanlines_in_current_frame: 0,
            bus: Some(bus),
        }
    }

    fn trigger_new_scanline_if_needed(&mut self) {
        if self.ppu_cycles_in_current_scanline >= SCANLINE_LENGTH_PIXELS {
            self.ppu_cycles_in_current_scanline -= SCANLINE_LENGTH_PIXELS;
            self.scanlines_in_current_frame += 1;
        }
    }

    fn is_sprite_0_hit(&self) -> bool {
        let sprite_0_x = bus!(self).ppu_memory.oam_data[0] as usize;
        let sprite_0_y = bus!(self).ppu_memory.oam_data[3] as usize;
        sprite_0_y == self.scanlines_in_current_frame
            && sprite_0_x == self.ppu_cycles_in_current_scanline
    }
    pub fn run_one_ppu_cycle(
        &mut self,
        texture: &mut Texture,
        frame: &mut Frame,
        canvas: &mut WindowCanvas,
        event_pump: &mut EventPump,
    ) {
        self.ppu_cycles_in_current_scanline += 1;
        // todo - actually draw something
        self.trigger_new_scanline_if_needed();
        if self.is_sprite_0_hit() {
            status_reg!(self).set_sprite_0_hit_status(true);
        }
        if self.scanlines_in_current_frame == NMI_SCANLINE
            && self.ppu_cycles_in_current_scanline == 0
        {
            self.handle_user_input(event_pump);
            bus_mut!(self).ppu_registers.copy_t_to_v();
            self.render_full_screen_background(texture, frame, canvas);
            status_reg!(self).set_sprite_0_hit_status(false);
            bus_mut!(self)
                .ppu_registers
                .status_register
                .set_vblank_status(true);
            if bus!(self).ppu_registers.control_register.get_vblank_nmi() {
                bus_mut!(self).nmi_generated = true;
            }
        }

        if self.scanlines_in_current_frame >= SCANLINES_PER_FRAME {
            self.scanlines_in_current_frame -= SCANLINES_PER_FRAME;

            status_reg!(self).set_sprite_0_hit_status(false);
            bus_mut!(self)
                .ppu_registers
                .status_register
                .set_vblank_status(false);
            // todo - disable nmi
        }
    }
}
