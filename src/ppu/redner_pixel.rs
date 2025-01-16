use super::PPU;
use crate::bus;
use crate::ppu::colors_palette::SYSTEM_PALETTE;
use crate::ppu::full_screen_rendering::{SCREEN_HEIGHT_TILE, SCREEN_WIDTH_TILE};
use crate::ppu::render_sdl::screen_rendering_constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::ppu::render_sdl::Frame;

impl<'bus> PPU<'bus> {
    pub fn render_pixel(&self, x: usize, y: usize, frame: &mut Frame) {
        if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
            return;
        }

        let x_offset = bus!(self).ppu_registers.scroll_register.get_x_scroll() as usize
            + bus!(self).ppu_registers.control_register.get_nametable_x() * SCREEN_WIDTH
            + x;
        let abs_x_offset = x_offset % (2 * SCREEN_WIDTH);
        let x_nametable = match abs_x_offset >= SCREEN_WIDTH {
            true => 1,
            false => 0,
        };
        let x_loc_in_nametable = abs_x_offset % SCREEN_WIDTH;
        let tile_x = x_loc_in_nametable / 8;
        let x_loc_in_tile = x_loc_in_nametable % 8;

        let y_offset = bus!(self).ppu_registers.scroll_register.get_y_scroll() as usize
            + bus!(self).ppu_registers.control_register.get_nametable_y() * SCREEN_HEIGHT
            + y;
        let abs_y_offset = y_offset % (2 * SCREEN_HEIGHT);
        let y_nametable = match abs_y_offset >= SCREEN_HEIGHT {
            true => 1,
            false => 0,
        };
        let y_loc_in_nametable = abs_y_offset % SCREEN_HEIGHT;
        let tile_y = y_loc_in_nametable / 8;
        let y_loc_in_tile = y_loc_in_nametable % 8;

        let nametable_base = x_nametable * 0x400 + y_nametable * 0x800;

        let tile_index = tile_y * SCREEN_WIDTH_TILE + tile_x;
        let tile_number = self.get_tile_number(tile_index, nametable_base);

        let tile = self.get_actual_tile_data(tile_number);

        let palette_index = self.get_palette_index(tile_x, tile_y, nametable_base);
        let tile_palette = self.get_background_palette(palette_index);

        self.draw_pixel(
            frame,
            y_loc_in_tile,
            x_loc_in_tile,
            y,
            x,
            tile,
            tile_palette,
        )
    }

    fn draw_pixel(
        &self,
        frame: &mut Frame,
        y_location_in_tile: usize,
        x_location_in_tile: usize,
        y_location_in_screen: usize,
        x_location_in_screen: usize,
        tile: &[u8],
        tile_palette: [u8; 4],
    ) {
        // if y_location_in_screen == 80 {
        //     frame.set_pixel(
        //         x_location_in_screen,
        //         y_location_in_screen,
        //         SYSTEM_PALETTE[0],
        //     );
        //     return;
        // }
        let upper_byte = tile[y_location_in_tile];
        let lower_byte = tile[y_location_in_tile + 8];

        let color_index_high = 1 & (lower_byte >> (7 - x_location_in_tile));
        let color_index_low = 1 & (upper_byte >> (7 - x_location_in_tile));
        let color_index = color_index_high << 1 | color_index_low;

        let rgb = SYSTEM_PALETTE[tile_palette[color_index as usize] as usize];

        // if y_location_in_screen == 60 && x_location_in_screen == 160 {
        //     let x_offset = bus!(self).ppu_registers.scroll_register.get_x_scroll() as usize
        //         + bus!(self).ppu_registers.control_register.get_nametable_x() * SCREEN_WIDTH;
        //     let y_offset = bus!(self).ppu_registers.scroll_register.get_y_scroll() as usize
        //         + bus!(self).ppu_registers.control_register.get_nametable_y() * SCREEN_HEIGHT;
        //     frame.set_pixel(
        //         x_location_in_screen,
        //         y_location_in_screen,
        //         SYSTEM_PALETTE[5],
        //     );
        //     return;
        // }

        frame.set_pixel(x_location_in_screen, y_location_in_screen, rgb)
    }
}
