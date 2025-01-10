use super::PPU;
use crate::ppu::colors_palette::SYSTEM_PALETTE;
use crate::ppu::render_sdl::screen_rendering_constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::ppu::render_sdl::{update_texture_from_frame, Frame};
use crate::{bus, palette, ppu_mem};
use sdl2::render::{Texture, WindowCanvas};

pub const SCREEN_WIDTH_TILE: usize = SCREEN_WIDTH / 8;
const SCREEN_HEIGHT_TILE: usize = SCREEN_HEIGHT / 8;
const SCREEN_SIZE_TILE: usize = SCREEN_WIDTH_TILE * SCREEN_HEIGHT_TILE;

impl<'bus> PPU<'bus> {
    pub fn copy_background_to_frame(&self, frame: &mut Frame) {
        for tile_x in 0..SCREEN_WIDTH_TILE {
            for tile_y in 0..SCREEN_HEIGHT_TILE {
                let tile_index = tile_y * SCREEN_WIDTH_TILE + tile_x;
                let tile_number = self.get_tile_number(tile_index);

                let tile = self.get_actual_tile_data(tile_number);

                let palette_index = self.get_palette_index(tile_x, tile_y);
                let tile_palette = self.get_background_palette(palette_index);

                frame.draw_tile_one_frame(tile_x, tile_y, tile, tile_palette);
            }
        }
    }

    fn get_background_palette(&self, palette_index: usize) -> [u8; 4] {
        let tile_palette = [
            palette!(self)[0],
            palette!(self)[palette_index * 4 + 1],
            palette!(self)[palette_index * 4 + 2],
            palette!(self)[palette_index * 4 + 3],
        ];
        tile_palette
    }

    fn get_palette_index(&self, tile_x: usize, tile_y: usize) -> usize {
        let meta_tile_index_for_color = tile_x / 4 + tile_y / 4 * SCREEN_WIDTH_TILE / 4;
        let attribute_color_byte =
            ppu_mem!(self).vram[SCREEN_SIZE_TILE + meta_tile_index_for_color];
        let palette_index_in_attribute_byte = match (tile_x % 4 < 2, tile_y % 4 < 2) {
            (true, true) => 0,
            (false, true) => 2,
            (true, false) => 4,
            (false, false) => 6,
        };
        let palette_index =
            (attribute_color_byte >> palette_index_in_attribute_byte) as usize & 0b11;
        palette_index
    }

    fn get_actual_tile_data(&self, tile_number: u16) -> &[u8] {
        let bank_start = bus!(self)
            .ppu_registers
            .control_register
            .clone()
            .get_background_sprite_address();
        let tile_start = (bank_start + tile_number * 16) as usize;
        let tile = &bus!(self).cartridge.chr_rom[tile_start..=tile_start + 15];
        tile
    }

    fn get_tile_number(&self, tile_index: usize) -> u16 {
        let tile_number = bus!(self).ppu_memory.vram[tile_index] as u16;
        tile_number
    }

    fn copy_sprite_to_frame(&self, frame: &mut Frame) {
        for i in (0..256).step_by(4).rev() {
            let tile_idx = ppu_mem!(self).oam_data[i + 1] as u16;
            let tile_x = ppu_mem!(self).oam_data[i + 3] as usize;
            let tile_y = ppu_mem!(self).oam_data[i] as usize;

            let flip_vertical = if ppu_mem!(self).oam_data[i + 2] >> 7 & 1 == 1 {
                true
            } else {
                false
            };
            let flip_horizontal = if ppu_mem!(self).oam_data[i + 2] >> 6 & 1 == 1 {
                true
            } else {
                false
            };
            let palette_idx = ppu_mem!(self).oam_data[i + 2] & 0b11;

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
