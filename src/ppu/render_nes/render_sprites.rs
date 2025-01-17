use super::PPU;
use crate::ppu::colors_palette::SYSTEM_PALETTE;
use crate::ppu::render_sdl::screen_rendering_constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::ppu::render_sdl::{update_texture_from_frame, Frame};
use crate::{bus, ppu_mem};
use sdl2::render::{Texture, WindowCanvas};

pub const SCREEN_WIDTH_TILE: usize = SCREEN_WIDTH / 8;
pub const SCREEN_HEIGHT_TILE: usize = SCREEN_HEIGHT / 8;
pub(crate) const SCREEN_SIZE_TILE: usize = SCREEN_WIDTH_TILE * SCREEN_HEIGHT_TILE;

impl<'bus> PPU<'bus> {
    pub(crate) fn copy_sprite_to_frame(&self, frame: &mut Frame) {
        for i in (0..256).step_by(4).rev() {
            // some sprites should not be rendered
            if (ppu_mem!(self).oam_data[i + 2] >> 5) & 1 != 0 {
                // sprite priority is 0 means we should skip it
                continue;
            }

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
        // frame.screen_state = [0; SCREEN_WIDTH * SCREEN_HEIGHT * 3];
        // self.copy_background_to_frame(frame);
        self.copy_sprite_to_frame(frame);
        update_texture_from_frame(texture, frame, canvas);
        canvas.present();
    }
}
