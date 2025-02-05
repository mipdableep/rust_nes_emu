use crate::ppu::render_nes::ppu_render_constants::*;

use crate::bus::{PPU_NAMETABLE_SIZE, PPU_NAMETABLE_START};
use crate::ppu::colors_palette::SYSTEM_PALETTE;

use crate::ppu::frame::Frame;
use crate::ppu::PPU;
use crate::{bus, bus_mut, palette};

macro_rules! status_reg {
    ($ppu: ident) => {
        bus_mut!($ppu).ppu_registers.status_register
    };
}

fn get_pixel_palette_index(nametable_high: u8, nametable_low: u8, location_in_raw: u8) -> usize {
    ((2 * ((nametable_high >> location_in_raw) & 1)) | ((nametable_low >> location_in_raw) & 1))
        as usize
}

fn fetch_background_8_pixels(
    nametable_high: u8,
    nametable_low: u8,
    palette: [usize; 4],
) -> [(u8, u8, u8); TILE_WIDTH] {
    [
        SYSTEM_PALETTE[palette[get_pixel_palette_index(nametable_high, nametable_low, 7)]],
        SYSTEM_PALETTE[palette[get_pixel_palette_index(nametable_high, nametable_low, 6)]],
        SYSTEM_PALETTE[palette[get_pixel_palette_index(nametable_high, nametable_low, 5)]],
        SYSTEM_PALETTE[palette[get_pixel_palette_index(nametable_high, nametable_low, 4)]],
        SYSTEM_PALETTE[palette[get_pixel_palette_index(nametable_high, nametable_low, 3)]],
        SYSTEM_PALETTE[palette[get_pixel_palette_index(nametable_high, nametable_low, 2)]],
        SYSTEM_PALETTE[palette[get_pixel_palette_index(nametable_high, nametable_low, 1)]],
        SYSTEM_PALETTE[palette[get_pixel_palette_index(nametable_high, nametable_low, 0)]],
    ]
}

fn nametable_base(tile_x: usize, tile_y: usize) -> usize {
    // return the nametable base address from tile_x and tile_y
    match (tile_x >= SCREEN_WIDTH_TILE, tile_y >= SCREEN_HEIGHT_TILE) {
        (false, false) => PPU_NAMETABLE_START as usize,
        (true, false) => (PPU_NAMETABLE_START + PPU_NAMETABLE_SIZE) as usize,
        (false, true) => (PPU_NAMETABLE_START + 2 * PPU_NAMETABLE_SIZE) as usize,
        (true, true) => (PPU_NAMETABLE_START + 3 * PPU_NAMETABLE_SIZE) as usize,
    }
}

fn tile_x_in_nametable(tile_x: usize) -> usize {
    // return the nametable base address from tile_x
    tile_x % SCREEN_WIDTH_TILE
}

fn tile_y_in_nametable(tile_y: usize) -> usize {
    // return the nametable base address from tile_y
    tile_y % SCREEN_HEIGHT_TILE
}

impl<'bus> PPU<'bus> {
    fn get_current_abs_x(&self) -> usize {
        let abs_x = self.ppu_cycles_in_current_scanline + self.cur_scanline_x_offset + TILE_WIDTH; // we add tile width because we render the first two line in the previous scanline
        abs_x % (2 * SCREEN_WIDTH)
    }

    fn get_current_abs_y(&self) -> usize {
        let abs_y = self.scanlines_in_current_frame + self.cur_scanline_y_offset;
        abs_y % (2 * SCREEN_HEIGHT)
    }

    fn get_next_y_line(&self) -> usize {
        let next_abs_y =
            ((self.scanlines_in_current_frame + 1) % SCREEN_HEIGHT) + self.cur_scanline_y_offset;
        next_abs_y % (2 * SCREEN_HEIGHT)
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
        y_offset_in_tile: usize,
    ) -> (u8, u8) {
        let current_tile_number = self.get_current_tile_number(tile_x, tile_y, nametable_base);
        let bank_start = bus!(self).ppu_registers.get_tile_background_tile_bank();

        let tile_start = (bank_start + current_tile_number * 16) as usize;

        (
            bus!(self).cartridge.chr_rom[tile_start + y_offset_in_tile + TILE_HEIGHT],
            bus!(self).cartridge.chr_rom[tile_start + y_offset_in_tile],
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

    fn get_tile_row_from_x_y(&self, abs_x: usize, abs_y: usize) -> [(u8, u8, u8); TILE_WIDTH] {
        // gets x,y in absolute nametable (x in 0..SCREEN_WIDTH*2, y in 0..SCREEN_HEIGHT*2)
        // and return the pixels of the relevant row
        let abs_tile_x = abs_x / TILE_WIDTH;
        let tile_x = tile_x_in_nametable(abs_tile_x);

        let abs_tile_y = abs_y / TILE_HEIGHT;
        let tile_y = tile_y_in_nametable(abs_tile_y);

        let tile_nametable = nametable_base(abs_tile_x, abs_tile_y);

        let (nametable_byte_high, nametable_byte_low) =
            self.fetch_nametable_bytes(tile_x, tile_y, tile_nametable, abs_y % TILE_HEIGHT);

        let attribute_byte = self.fetch_attribute_byte(tile_x, tile_y, tile_nametable);
        let palette_index = self.get_current_palette_index(tile_x, tile_y, attribute_byte);
        let palette = self.fetch_current_palette(palette_index);

        fetch_background_8_pixels(nametable_byte_high, nametable_byte_low, palette)
    }

    fn get_current_tile_row_pixels(&self) -> [(u8, u8, u8); TILE_WIDTH] {
        let current_abs_x = self.get_current_abs_x();
        let current_abs_y = self.get_current_abs_y();

        self.get_tile_row_from_x_y(current_abs_x, current_abs_y)
    }

    fn set_background_pixel(
        &mut self,
        frame: &mut Frame,
        x: usize,
        y: usize,
        bkg_color: (u8, u8, u8),
    ) {
        // this function compares the pixel with the sprites pixel
        // and handles background/foreground correctly
        match self.next_line_sprite_pixels[x] {
            None => frame.set_pixel(x, y, bkg_color),
            Some(sprite_pixel) => {
                match sprite_pixel.is_background {
                    true => match bkg_color != SYSTEM_PALETTE[palette!(self)[0] as usize] {
                        true => frame.set_pixel(x, y, bkg_color),
                        false => frame.set_pixel(x, y, sprite_pixel.color),
                    },
                    false => frame.set_pixel(x, y, sprite_pixel.color),
                }

                match sprite_pixel.sprite_index == 0 {
                    true => {
                        status_reg!(self).set_sprite_0_hit_status(true);
                    }
                    false => {}
                }
            }
        }
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

                    let row_tile_pixels = self.get_current_tile_row_pixels();

                    let fine_x_shift = self.cur_scanline_x_offset % TILE_WIDTH;
                    // we must set the right pixels
                    // since the first 2 tiles are fetched during the end of the previous scanline,
                    // we only need to handle the end of the row
                    let tile_left_position_in_frame = x_pos_in_frame + TILE_WIDTH - fine_x_shift;
                    for i in 0..TILE_WIDTH {
                        if tile_left_position_in_frame + i >= SCREEN_WIDTH {
                            // we "overshoot"
                            break;
                        }
                        self.set_background_pixel(
                            frame,
                            tile_left_position_in_frame + i,
                            row_number,
                            row_tile_pixels[i],
                        );
                    }
                }
            }
            SCREEN_WIDTH => {
                // increase coarse x and coarse y
                // we will handle it as nop
            }
            const { SCREEN_WIDTH + 1 } => {
                // we will re-calculate the offset each scanline
                self.cur_scanline_x_offset = bus!(self).ppu_registers.get_abs_x_offset()
            }
            const { SCREEN_WIDTH + 2 }..DOT_TO_START_FETCH_NEXT_LINE_TILES => {} //garbage nametable fetch
            DOT_TO_START_FETCH_NEXT_LINE_TILES..FIRST_TILE_FETCH_DOT => {}
            FIRST_TILE_FETCH_DOT => {
                // fetch the tile
                let row_number = (self.scanlines_in_current_frame + 1) % SCREEN_HEIGHT;

                let row_tile_pixels = self.get_tile_row_from_x_y(
                    self.cur_scanline_x_offset % (2 * SCREEN_WIDTH), // the left x pos of next scanline
                    self.get_next_y_line(),
                );

                let fine_x_shift = self.cur_scanline_x_offset % TILE_WIDTH;
                for i in 0..TILE_WIDTH - fine_x_shift {
                    self.set_background_pixel(
                        frame,
                        i,
                        row_number,
                        row_tile_pixels[i + fine_x_shift],
                    )
                }
            }
            const { FIRST_TILE_FETCH_DOT + 1 }..const { FIRST_TILE_FETCH_DOT + TILE_WIDTH } => {}
            const { FIRST_TILE_FETCH_DOT + TILE_WIDTH } => {
                // fetch the second tile
                let row_number = (self.scanlines_in_current_frame + 1) % SCREEN_HEIGHT;

                let row_tile_pixels = self.get_tile_row_from_x_y(
                    (self.cur_scanline_x_offset + TILE_WIDTH) % (2 * SCREEN_WIDTH), // the left x pos of next scanline second tile
                    self.get_next_y_line(),
                );

                let fine_x_shift = self.cur_scanline_x_offset % TILE_WIDTH;
                for i in 0..TILE_WIDTH {
                    self.set_background_pixel(
                        frame,
                        8 - fine_x_shift + i,
                        row_number,
                        row_tile_pixels[i],
                    )
                }
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
            // "copy loopy_t y to loopy v"
            self.cur_scanline_y_offset = bus!(self).ppu_registers.get_abs_y_offset()
        }
        if self.ppu_cycles_in_current_scanline > SCREEN_WIDTH {
            self.handle_visible_scanline(frame);
        }
    }

    pub fn handle_background_one_cycle(&mut self, frame: &mut Frame) {
        match self.scanlines_in_current_frame {
            0..SCREEN_HEIGHT => {
                self.handle_visible_scanline(frame);
            }
            SCREEN_HEIGHT => {}
            NMI_SCANLINE => self.handle_post_render_scanline(),
            const { NMI_SCANLINE + 1 }..const { SCANLINES_PER_FRAME - 1 } => {}
            const { SCANLINES_PER_FRAME - 1 } => self.handle_pre_render_scanline(frame),
            _ => panic!("Shouldn't be here!"),
        }
    }
}
