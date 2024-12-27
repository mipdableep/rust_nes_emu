use super::PPU;
use crate::ppu::colors_palette::SYSTEM_PALETTE;
use crate::ppu::render_sdl::screen_rendering_constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::ppu::render_sdl::{update_texture_from_frame, Frame};
use sdl2::render::{Texture, WindowCanvas};

macro_rules! bus {
    ($self: ident) => {
        $self.bus.as_ref().unwrap()
    };
}

macro_rules! ppu_mem {
    ($self: ident) => {
        $self.bus.as_ref().unwrap().ppu_memory
    };
}

const SCREEN_WIDTH_TILE: usize = SCREEN_WIDTH / 8;
const SCREEN_HEIGHT_TILE: usize = SCREEN_HEIGHT / 8;
const SCREEN_SIZE_TILE: usize = SCREEN_WIDTH_TILE * SCREEN_HEIGHT_TILE;

impl<'bus> PPU<'bus> {
    fn copy_background_to_frame(&self, frame: &mut Frame) {
        for tile_index in 0..SCREEN_SIZE_TILE {
            // for tile_index in (0..960) {
            // just for now, lets use the first nametable
            let name_table_start = 0;
            let tile_number =
                self.bus.as_ref().unwrap().ppu_memory.vram[name_table_start + tile_index] as u16;
            let tile_x = tile_index % SCREEN_WIDTH_TILE;
            let tile_y = tile_index / SCREEN_WIDTH_TILE;
            let bank_start = bus!(self)
                .ppu_registers
                .control_register
                .clone()
                .get_background_sprite_address();
            let tile_start = (bank_start + tile_number * 16) as usize;
            let tile = &self.bus.as_ref().unwrap().cartridge.chr_rom[tile_start..=tile_start + 15];

            let meta_tile_index_for_color = tile_x / 4 + tile_y / 4 * SCREEN_WIDTH_TILE / 4;
            let attribute_color_byte = bus!(self).ppu_memory.vram
                [name_table_start + SCREEN_SIZE_TILE + meta_tile_index_for_color];
            let palette_index_in_attribute_byte = match (tile_x % 2, tile_y % 2) {
                (0, 0) => 0,
                (0, 1) => 2,
                (1, 0) => 4,
                (1, 1) => 6,
                (_, _) => {
                    panic!("Can't be! something % 2 is not 0 or 1!")
                }
            };
            let palette_index =
                (attribute_color_byte >> palette_index_in_attribute_byte) as usize & 0b11;
            let tile_palette =
                &bus!(self).ppu_memory.palette_table[palette_index * 4..(palette_index + 1) * 4];
            macro_rules! palette {
                () => {
                    bus!(self).ppu_memory.palette_table
                };
            }
            let tile_palette = [
                palette!()[0],
                palette!()[palette_index * 4 + 1],
                palette!()[palette_index * 4 + 2],
                palette!()[palette_index * 4 + 3],
            ];

            for y in 0..=7 {
                let mut upper = tile[y];
                let mut lower = tile[y + 8];

                for x in (0..=7).rev() {
                    let value = (1 & upper) << 1 | (1 & lower);
                    upper = upper >> 1;
                    lower = lower >> 1;
                    let rgb = SYSTEM_PALETTE[tile_palette[value as usize] as usize];
                    frame.set_pixel(tile_x * 8 + x, tile_y * 8 + y, rgb)
                }
            }
        }
    }

    fn copy_sprite_to_frame(&self, frame: &mut Frame) {
        for i in (0..256).step_by(4).rev() {
            let tile_idx = ppu_mem!(self).oam_data[i + 1] as u16;
            let tile_x = self.bus.as_ref().unwrap().ppu_memory.oam_data[i + 3] as usize;
            let tile_y = self.bus.as_ref().unwrap().ppu_memory.oam_data[i] as usize;

            let flip_vertical =
                if self.bus.as_ref().unwrap().ppu_memory.oam_data[i + 2] >> 7 & 1 == 1 {
                    true
                } else {
                    false
                };
            let flip_horizontal =
                if self.bus.as_ref().unwrap().ppu_memory.oam_data[i + 2] >> 6 & 1 == 1 {
                    true
                } else {
                    false
                };
            let palette_idx = self.bus.as_ref().unwrap().ppu_memory.oam_data[i + 2] & 0b11;

            let start = 0x11 + (palette_idx * 4) as usize;
            let sprite_palette = [
                0,
                ppu_mem!(self).palette_table[start],
                ppu_mem!(self).palette_table[start + 1],
                ppu_mem!(self).palette_table[start + 2],
            ];

            let bank: u16 = bus!(self)
                .ppu_registers
                .control_register
                .clone()
                .get_sprite_pattern_address();

            let tile = &bus!(self).cartridge.chr_rom
                [(bank + tile_idx * 16) as usize..=(bank + tile_idx * 16 + 15) as usize];

            for y in 0..=7 {
                let mut upper = tile[y];
                let mut lower = tile[y + 8];
                'painting: for x in (0..=7).rev() {
                    let value = (1 & lower) << 1 | (1 & upper);
                    upper = upper >> 1;
                    lower = lower >> 1;
                    let rgb = match value {
                        0 => continue 'painting, // skip coloring the pixel
                        1 => SYSTEM_PALETTE[sprite_palette[1] as usize],
                        2 => SYSTEM_PALETTE[sprite_palette[2] as usize],
                        3 => SYSTEM_PALETTE[sprite_palette[3] as usize],
                        _ => panic!("can't be"),
                    };
                    match (flip_horizontal, flip_vertical) {
                        (false, false) => frame.set_pixel(tile_x + x, tile_y + y, rgb),
                        (true, false) => frame.set_pixel(tile_x + 7 - x, tile_y + y, rgb),
                        (false, true) => frame.set_pixel(tile_x + x, tile_y + 7 - y, rgb),
                        (true, true) => frame.set_pixel(tile_x + 7 - x, tile_y + 7 - y, rgb),
                    }
                }
            }
        }
    }
    pub fn render_full_screen_background(
        &self,
        texture: &mut Texture,
        frame: &mut Frame,
        canvas: &mut WindowCanvas,
    ) {
        frame.screen_state = [0; SCREEN_WIDTH * SCREEN_HEIGHT * 3];
        self.copy_background_to_frame(frame);
        self.copy_sprite_to_frame(frame);
        update_texture_from_frame(texture, frame, canvas);
        canvas.present();
    }
}
