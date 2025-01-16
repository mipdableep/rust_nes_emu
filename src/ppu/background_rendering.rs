use super::{NMI_SCANLINE, PPU, SCANLINE_LENGTH_PIXELS};
use crate::ppu::colors_palette::SYSTEM_PALETTE;
use crate::ppu::full_screen_rendering::{SCREEN_SIZE_TILE, SCREEN_WIDTH_TILE};
use crate::ppu::render_sdl::screen_rendering_constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::ppu::render_sdl::Frame;
use crate::{bus, bus_mut, palette};

const TILE_WIDTH: usize = 8;
const TILE_HEIGHT: usize = 8;

const PRE_RENDER_SCANLINE: usize = 261;

const COPY_VERT_V_DOT_START: usize = 280;
const COPY_VERT_V_DOT_END: usize = 304;
const DOT_TO_START_FETCH_NEXT_LINE_TILES: usize = 321;
const FIRST_TILE_FETCH_DOT: usize = 328;

macro_rules! status_reg {
    ($ppu: ident) => {
        bus_mut!($ppu).ppu_registers.status_register
    };
}

fn fetch_background_8_pixels(
    nametable_high: u8,
    nametable_low: u8,
    palette: [usize; 4],
) -> [(u8, u8, u8); TILE_WIDTH] {
    [
        SYSTEM_PALETTE
            [palette[((2 * ((nametable_high >> 7) & 1)) | ((nametable_low >> 7) & 1)) as usize]],
        SYSTEM_PALETTE
            [palette[((2 * ((nametable_high >> 6) & 1)) | ((nametable_low >> 6) & 1)) as usize]],
        SYSTEM_PALETTE
            [palette[((2 * ((nametable_high >> 5) & 1)) | ((nametable_low >> 5) & 1)) as usize]],
        SYSTEM_PALETTE
            [palette[((2 * ((nametable_high >> 4) & 1)) | ((nametable_low >> 4) & 1)) as usize]],
        SYSTEM_PALETTE
            [palette[((2 * ((nametable_high >> 3) & 1)) | ((nametable_low >> 3) & 1)) as usize]],
        SYSTEM_PALETTE
            [palette[((2 * ((nametable_high >> 2) & 1)) | ((nametable_low >> 2) & 1)) as usize]],
        SYSTEM_PALETTE
            [palette[((2 * ((nametable_high >> 1) & 1)) | ((nametable_low >> 1) & 1)) as usize]],
        SYSTEM_PALETTE
            [palette[((2 * ((nametable_high >> 0) & 1)) | ((nametable_low >> 0) & 1)) as usize]],
    ]
}

impl<'bus> PPU<'bus> {
    fn get_current_tile_x(&self) -> usize {
        bus!(self).ppu_registers.get_coarse_x()
    }

    fn get_current_tile_y(&self) -> usize {
        bus!(self).ppu_registers.get_coarse_y()
    }

    fn get_current_nametable_base(&self) -> usize {
        bus!(self).ppu_registers.get_nametable_base_index()
    }

    fn get_current_tile_number(&self, tile_x: usize, tile_y: usize, nametable_base: usize) -> u16 {
        let tile_index = tile_y * SCREEN_WIDTH_TILE + tile_x;
        bus!(self).read_vram(tile_index + nametable_base) as u16
    }

    pub fn fetch_nametable_bytes(
        &self,
        tile_x: usize,
        tile_y: usize,
        nametable_base: usize,
    ) -> (u8, u8) {
        let current_tile_number = self.get_current_tile_number(tile_x, tile_y, nametable_base);
        let bank_start = bus!(self).ppu_registers.get_tile_background_tile_bank();

        let tile_start = (bank_start + current_tile_number * 16) as usize;
        let y_offset = bus!(self).ppu_registers.get_fine_y();

        (
            bus!(self).cartridge.chr_rom[tile_start + y_offset + TILE_HEIGHT],
            bus!(self).cartridge.chr_rom[tile_start + y_offset],
        )
    }

    pub fn fetch_attribute_byte(&self, tile_x: usize, tile_y: usize, nametable_base: usize) -> u8 {
        let meta_tile_index_for_color = tile_x / 4 + tile_y / 4 * SCREEN_WIDTH_TILE / 4;
        bus!(self).read_vram(SCREEN_SIZE_TILE + meta_tile_index_for_color + nametable_base)
    }

    fn get_current_palette_index(&self, tile_x: usize, tile_y: usize, attribute_byte: u8) -> usize {
        let palette_index_in_attribute_byte = match (tile_x % 4 < 2, tile_y % 4 < 2) {
            (true, true) => 0,
            (false, true) => 2,
            (true, false) => 4,
            (false, false) => 6,
        };
        let palette_index = (attribute_byte >> palette_index_in_attribute_byte) as usize & 0b11;
        palette_index
    }

    fn fetch_current_palette(&self, palette_index: usize) -> [usize; 4] {
        [
            palette!(self)[0] as usize,
            palette!(self)[palette_index * 4 + 1] as usize,
            palette!(self)[palette_index * 4 + 2] as usize,
            palette!(self)[palette_index * 4 + 3] as usize,
        ]
    }

    fn increase_current_x_one_tile(&mut self) {
        bus_mut!(self).ppu_registers.increase_coarse_x()
    }

    fn increase_current_y_one_row(&mut self) {
        bus_mut!(self).ppu_registers.increase_y()
    }

    fn get_current_tile_row_pixels(&self) -> [(u8, u8, u8); TILE_WIDTH] {
        let tile_nametable = self.get_current_nametable_base();
        let tile_x = self.get_current_tile_x();
        let tile_y = self.get_current_tile_y();
        let (nametable_byte_high, nametable_byte_low) =
            self.fetch_nametable_bytes(tile_x, tile_y, tile_nametable);

        let attribute_byte = self.fetch_attribute_byte(tile_x, tile_y, tile_nametable);
        let palette_index = self.get_current_palette_index(tile_x, tile_y, attribute_byte);
        let palette = self.fetch_current_palette(palette_index);

        fetch_background_8_pixels(nametable_byte_high, nametable_byte_low, palette)
    }

    fn handle_visible_scanline(&mut self, frame: &mut Frame) {
        let x_pos_in_frame = self.ppu_cycles_in_current_scanline;
        match x_pos_in_frame {
            0 => {} // I think we can treat it as a nop?
            1..SCREEN_WIDTH => {
                // for simplicity, we will "bulk fetch" everything every 8 cycles
                // instead of the more accurate dot-by-dot emulation
                if x_pos_in_frame % TILE_WIDTH == 0 {
                    let row_number = self.scanlines_in_current_frame;

                    if row_number == 33 {
                        if x_pos_in_frame + TILE_WIDTH == 72 {
                            println!("Here!");
                        }
                    }

                    let row_tile_pixels = self.get_current_tile_row_pixels();

                    let fine_x_shift = bus!(self).ppu_registers.get_fine_x();
                    // we must set the right pixels
                    // since the first 2 tiles are fetched during the end of the previous scanline,
                    // we only need to handle the end of the row
                    let tile_left_position_in_frame = x_pos_in_frame + TILE_WIDTH - fine_x_shift;
                    for i in 0..TILE_WIDTH {
                        if tile_left_position_in_frame + i >= SCREEN_WIDTH {
                            // we "overshoot"
                            break;
                        }
                        frame.set_pixel(
                            tile_left_position_in_frame + i,
                            row_number,
                            row_tile_pixels[i],
                        );
                    }
                    self.increase_current_x_one_tile();
                }
            }
            SCREEN_WIDTH => {
                // since this is an unused tile, we can approximate it by just increasing the x pos
                self.increase_current_x_one_tile();
                self.increase_current_y_one_row();
            }
            const { SCREEN_WIDTH + 1 } => bus_mut!(self).ppu_registers.copy_t_x_to_v(),
            const { SCREEN_WIDTH + 2 }..DOT_TO_START_FETCH_NEXT_LINE_TILES => {} //garbage nametable fetch
            DOT_TO_START_FETCH_NEXT_LINE_TILES..FIRST_TILE_FETCH_DOT => {}
            FIRST_TILE_FETCH_DOT => {
                // fetch the tile
                let row_number = (self.scanlines_in_current_frame + 1) % SCREEN_HEIGHT;

                let row_tile_pixels = self.get_current_tile_row_pixels();

                let fine_x_shift = bus!(self).ppu_registers.get_fine_x();
                for i in 0..TILE_WIDTH - fine_x_shift {
                    frame.set_pixel(i, row_number, row_tile_pixels[i + fine_x_shift])
                }
                self.increase_current_x_one_tile();
            }
            const { FIRST_TILE_FETCH_DOT + 1 }..const { FIRST_TILE_FETCH_DOT + TILE_WIDTH } => {}
            const { FIRST_TILE_FETCH_DOT + TILE_WIDTH } => {
                // fetch the second tile
                let row_number = (self.scanlines_in_current_frame + 1) % SCREEN_HEIGHT;

                let row_tile_pixels = self.get_current_tile_row_pixels();

                let fine_x_shift = bus!(self).ppu_registers.get_fine_x();
                for i in 0..TILE_WIDTH {
                    frame.set_pixel(8 - fine_x_shift + i, row_number, row_tile_pixels[i])
                }
                self.increase_current_x_one_tile();
            }
            const { FIRST_TILE_FETCH_DOT + TILE_WIDTH + 1 }..SCANLINE_LENGTH_PIXELS => {}
            _ => {
                panic!("Reached impossible dot in visible scanline!")
            }
        }
    }

    fn handle_post_render_scanline(&mut self) {
        if self.ppu_cycles_in_current_scanline == 1 {
            bus_mut!(self)
                .ppu_registers
                .status_register
                .set_vblank_status(true);

            if bus!(self).ppu_registers.control_register.get_vblank_nmi() {
                bus_mut!(self).nmi_generated = true;
            }
        }
    }

    fn handle_pre_render_scanline(&mut self, frame: &mut Frame) {
        // mostly the same as a visible frame, but we copy vert v to vert t
        if self.ppu_cycles_in_current_scanline == 1 {
            // clear vblank and sprite 0 hit
            status_reg!(self).set_sprite_0_hit_status(false);
            bus_mut!(self)
                .ppu_registers
                .status_register
                .set_vblank_status(false);
        }

        if COPY_VERT_V_DOT_START <= self.ppu_cycles_in_current_scanline
            && self.ppu_cycles_in_current_scanline <= COPY_VERT_V_DOT_END
        {
            bus_mut!(self).ppu_registers.copy_t_y_to_v();
        }
        if self.ppu_cycles_in_current_scanline > SCREEN_WIDTH {
            self.handle_visible_scanline(frame);
        }
    }

    fn is_sprite_0_hit(&self) -> bool {
        let sprite_0_y = bus!(self).ppu_memory.oam_data[0] as usize;
        let sprite_0_x = bus!(self).ppu_memory.oam_data[3] as usize;
        sprite_0_y == self.scanlines_in_current_frame
            && sprite_0_x == self.ppu_cycles_in_current_scanline
    }

    pub fn handle_background_one_cycle(&mut self, frame: &mut Frame) {
        if self.is_sprite_0_hit() {
            status_reg!(self).set_sprite_0_hit_status(true);
        }
        match self.scanlines_in_current_frame {
            0..SCREEN_HEIGHT => {
                self.handle_visible_scanline(frame);
            }
            SCREEN_HEIGHT => {}
            NMI_SCANLINE => self.handle_post_render_scanline(),
            const { NMI_SCANLINE + 1 }..PRE_RENDER_SCANLINE => {}
            PRE_RENDER_SCANLINE => self.handle_pre_render_scanline(frame),
            _ => panic!("Shouldn't be here!"),
        }
    }
}
