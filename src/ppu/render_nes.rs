mod render_background;
mod render_sprites;

use super::PPU;
use crate::ppu::render_sdl::screen_rendering_constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::{bus, palette};

pub const SCREEN_WIDTH_TILE: usize = SCREEN_WIDTH / 8;
pub const SCREEN_HEIGHT_TILE: usize = SCREEN_HEIGHT / 8;
pub(crate) const SCREEN_SIZE_TILE: usize = SCREEN_WIDTH_TILE * SCREEN_HEIGHT_TILE;

impl<'bus> PPU<'bus> {
    pub(crate) fn get_background_palette(&self, palette_index: usize) -> [u8; 4] {
        let tile_palette = [
            palette!(self)[0],
            palette!(self)[palette_index * 4 + 1],
            palette!(self)[palette_index * 4 + 2],
            palette!(self)[palette_index * 4 + 3],
        ];
        tile_palette
    }

    pub(crate) fn get_palette_index(
        &self,
        tile_x: usize,
        tile_y: usize,
        nametable_base: usize,
    ) -> usize {
        let meta_tile_index_for_color = tile_x / 4 + tile_y / 4 * SCREEN_WIDTH_TILE / 4;
        let attribute_color_byte =
            bus!(self).read_vram(SCREEN_SIZE_TILE + meta_tile_index_for_color + nametable_base);
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

    pub(crate) fn get_actual_tile_data(&self, tile_number: u16) -> &[u8] {
        let bank_start = bus!(self)
            .ppu_registers
            .control_register
            .clone()
            .get_background_sprite_address();
        let tile_start = (bank_start + tile_number * 16) as usize;
        let tile = &bus!(self).cartridge.chr_rom[tile_start..=tile_start + 15];
        tile
    }

    pub(crate) fn get_tile_number(&self, tile_index: usize, nametable_base: usize) -> u16 {
        let tile_number = bus!(self).read_vram(tile_index + nametable_base) as u16;
        tile_number
    }
}
