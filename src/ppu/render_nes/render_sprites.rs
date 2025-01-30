use super::PPU;
use crate::ppu::colors_palette::SYSTEM_PALETTE;
use crate::ppu::frame::{update_texture_from_frame, Frame};
use crate::ppu::render_nes::ppu_render_constants::{SCANLINE_LENGTH_PIXELS, TILE_HEIGHT};
use crate::ppu::{MAX_SPRITES_PER_LINE, SCREEN_HEIGHT};
use crate::{bus, bus_mut, ppu_mem};
use sdl2::render::{Texture, WindowCanvas};

macro_rules! status_reg {
    ($ppu: ident) => {
        bus_mut!($ppu).ppu_registers.status_register
    };
}

impl<'bus> PPU<'bus> {
    fn clear_secondary_oam(&mut self) {
        // happens in cycles 1 - 64
        if self.ppu_cycles_in_current_scanline % 2 == 0 {
            self.secondary_oam[self.ppu_cycles_in_current_scanline / 2 - 1] = 0xFF;
            // maybe we should also return FF when reading 0x2004?
        }
    }

    fn sprite_evaluation(&mut self) {
        // happens in cycles 65 - 256
        // we will first implement it happening at once (simultaneously do the actions of all the cycles)
        let mut number_of_sprites_in_scanline = 0;
        for n in 0..64 {
            // n is the sprite number
            let sprite_y = bus!(self).ppu_memory.oam_data[4 * n] as usize;
            if !(sprite_y <= self.scanlines_in_current_frame
                && self.scanlines_in_current_frame < sprite_y + TILE_HEIGHT)
            {
                // sprite is not relevant to this scanline
                continue;
            }
            if number_of_sprites_in_scanline >= MAX_SPRITES_PER_LINE {
                // we will implement the buggy behaviour of the hardware
                // instead, we will implement it correctly
                bus_mut!(self)
                    .ppu_registers
                    .status_register
                    .set_sprite_overload(true);
                continue;
            }
            // copy the sprite
            self.secondary_oam[4 * number_of_sprites_in_scanline] =
                bus!(self).ppu_memory.oam_data[4 * n];
            self.secondary_oam[4 * number_of_sprites_in_scanline + 1] =
                bus!(self).ppu_memory.oam_data[4 * n + 1];
            self.secondary_oam[4 * number_of_sprites_in_scanline + 2] =
                bus!(self).ppu_memory.oam_data[4 * n + 2];
            self.secondary_oam[4 * number_of_sprites_in_scanline + 3] =
                bus!(self).ppu_memory.oam_data[4 * n + 3];

            number_of_sprites_in_scanline += 1;
            if n == 0 {
                self.sprite_0_hit_this_scanline = true;
            }
        }
    }

    fn get_sprite_palette(&self, attribute_byte: u8) -> [u8; 4] {
        let palette_idx = (attribute_byte & 0b11) as usize;
        let start = 0x11 + (palette_idx * 4);
        [
            0,
            ppu_mem!(self).palette_table[start + 0],
            ppu_mem!(self).palette_table[start + 1],
            ppu_mem!(self).palette_table[start + 2],
        ]
    }

    fn get_sprite_bank_start(&self) -> u16 {
        bus!(self)
            .ppu_registers
            .control_register
            .clone()
            .get_sprite_pattern_address()
    }

    fn fetch_sprite_nametable_bytes(
        &self,
        tile_number: usize,
        y_offset_in_tile: usize,
    ) -> (u8, u8) {
        let bank_start = bus!(self).ppu_registers.get_sprite_tile_bank() as usize;

        let tile_start = bank_start + tile_number * 16;

        (
            bus!(self).cartridge.chr_rom[tile_start + y_offset_in_tile + TILE_HEIGHT],
            bus!(self).cartridge.chr_rom[tile_start + y_offset_in_tile],
        )
    }

    fn draw_sprites(&mut self, frame: &mut Frame, sprite_number: usize) {
        let sprite_y = self.secondary_oam[4 * sprite_number] as usize;
        let tile_number = self.secondary_oam[4 * sprite_number + 1] as usize;
        let attribute_byte = self.secondary_oam[4 * sprite_number + 2];
        let sprite_x = self.secondary_oam[4 * sprite_number + 3] as usize;

        if sprite_number == 0 && self.sprite_0_hit_this_scanline {
            if self.scanlines_in_current_frame == sprite_y + 5 {
                status_reg!(self).set_sprite_0_hit_status(true);
            }
        }

        if attribute_byte >> 5 & 1 != 0 {
            // sprite priority is 0 means we should skip it
            return;
        }

        let flip_vertical = attribute_byte >> 7 & 1 == 1;
        let flip_horizontal = attribute_byte >> 6 & 1 == 1;

        let y_offset_in_tile = match flip_vertical {
            true => 7 - (self.scanlines_in_current_frame - sprite_y),
            false => self.scanlines_in_current_frame - sprite_y,
        };

        let (mut nametable_byte_low, mut nametable_byte_high) =
            self.fetch_sprite_nametable_bytes(tile_number, y_offset_in_tile);

        let sprite_palette = self.get_sprite_palette(attribute_byte);

        'painting: for x in (0..=7).rev() {
            let value = (1 & nametable_byte_low) << 1 | (1 & nametable_byte_high);
            nametable_byte_high = nametable_byte_high >> 1;
            nametable_byte_low = nametable_byte_low >> 1;
            let rgb = match value {
                0 => continue 'painting, // skip coloring the pixel
                1 => SYSTEM_PALETTE[sprite_palette[1] as usize],
                2 => SYSTEM_PALETTE[sprite_palette[2] as usize],
                3 => SYSTEM_PALETTE[sprite_palette[3] as usize],
                _ => panic!("can't be"),
            };
            match flip_horizontal {
                false => frame.set_pixel(sprite_x + x, self.scanlines_in_current_frame, rgb),
                true => frame.set_pixel(sprite_x + 7 - x, self.scanlines_in_current_frame, rgb),
            }
        }
    }

    fn handle_sprites_one_cycle_visible_scanline(&mut self, frame: &mut Frame) {
        match self.ppu_cycles_in_current_scanline {
            0 => {}
            1..=64 => self.clear_secondary_oam(),
            65..256 => {}
            256 => self.sprite_evaluation(),
            257..=320 => {
                if self.ppu_cycles_in_current_scanline % 8 == 1 {
                    let sprite_number = (self.ppu_cycles_in_current_scanline - 257) / 8;
                    self.draw_sprites(frame, sprite_number);
                }
            }
            321..SCANLINE_LENGTH_PIXELS => {}
            _ => panic!("Shouldn't be! {}", self.ppu_cycles_in_current_scanline),
        }
    }

    pub fn handle_sprites_one_cycle(&mut self, frame: &mut Frame) {
        if self.scanlines_in_current_frame < SCREEN_HEIGHT {
            self.handle_sprites_one_cycle_visible_scanline(frame);
        }
    }
}
