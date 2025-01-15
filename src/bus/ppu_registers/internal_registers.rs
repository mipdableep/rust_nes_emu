#[cfg(test)]
mod address;
#[cfg(test)]
mod scroll;
#[cfg(test)]
mod testing_internal;

use ppu_masks::*;

#[derive(Debug, Eq, PartialEq)]
pub struct InternalPPURegisters {
    // based off https://www.nesdev.org/wiki/PPU_scrolling
    current_vram: u16,      // v
    temporary_vram: u16,    // t,
    fine_x: u8,             //x, only 3 bits
    pub write_toggle: bool, // w
}

/// Structure of both v and t is
/// EDC BA 98765 43210
/// yyy NN YYYYY XXXXX
/// ||| || ||||| +++++-- coarse X scroll
/// ||| || +++++-------- coarse Y scroll
/// ||| ++-------------- nametable select
/// +++----------------- fine Y scroll
///
/// We ignore rustfmt to allow nice alignment
#[cfg_attr(rustfmt, rustfmt_skip)]
mod ppu_masks{
    pub const FINE_Y_MASK: u16 =    0b0_111_00_00000_00000;
    pub const NAMETABLE_MASK: u16 = 0b0_000_11_00000_00000;
    pub const COARSE_Y_MASK: u16 =  0b0_000_00_11111_00000;
    pub const COARSE_X_MASK: u16 =  0b0_000_00_00000_11111;
}

fn get_fine_y(ppu_vram_addr: u16) -> u8 {
    ((ppu_vram_addr & FINE_Y_MASK) >> 0xC) as u8
}

fn set_fine_y(ppu_vram_addr: &mut u16, fine_y_value: u8) {
    let cleaned_y_value = (fine_y_value as u16 & 0b111) << 0xC;
    *ppu_vram_addr = (*ppu_vram_addr & !FINE_Y_MASK) | (cleaned_y_value & FINE_Y_MASK)
}

fn get_nametable(ppu_vram_addr: u16) -> u8 {
    ((ppu_vram_addr & NAMETABLE_MASK) >> 0xA) as u8
}

fn set_nametable(ppu_vram_addr: &mut u16, nametable_value: u8) {
    let cleaned_nametable_value = (nametable_value as u16 & 0b11) << 0xA;
    *ppu_vram_addr = (*ppu_vram_addr & !NAMETABLE_MASK) | (cleaned_nametable_value & NAMETABLE_MASK)
}

fn get_coarse_y(ppu_vram_addr: u16) -> u8 {
    ((ppu_vram_addr & COARSE_Y_MASK) >> 0x5) as u8
}

fn set_coarse_y(ppu_vram_addr: &mut u16, coarse_y_value: u8) {
    let cleaned_coarse_y = (coarse_y_value as u16 & 0b11111) << 5;
    *ppu_vram_addr = (*ppu_vram_addr & !COARSE_Y_MASK) | (cleaned_coarse_y & COARSE_Y_MASK)
}

fn get_coarse_x(ppu_vram_addr: u16) -> u8 {
    (ppu_vram_addr & COARSE_X_MASK) as u8
}

fn set_coarse_x(ppu_vram_addr: &mut u16, coarse_x_value: u8) {
    let cleaned_coarse_x = coarse_x_value as u16 & 0b11111;
    *ppu_vram_addr = (*ppu_vram_addr & !COARSE_X_MASK) | (cleaned_coarse_x & COARSE_X_MASK)
}

impl InternalPPURegisters {
    pub fn new() -> Self {
        Self {
            current_vram: 0,
            temporary_vram: 0,
            fine_x: 0,
            write_toggle: false,
        }
    }

    pub fn write_control(&mut self, control_value: u8) {
        // t: ...GH.. ........ <- d: ......GH
        set_nametable(&mut self.temporary_vram, control_value)
    }

    pub fn reset_toggle(&mut self) {
        self.write_toggle = false
    }

    pub fn set_toggle(&mut self) {
        self.write_toggle = true
    }

    pub fn write_scroll(&mut self, scroll_value: u8) {
        match self.write_toggle {
            true => {
                // if w i 1
                // t: FGH..AB CDE..... <- d: ABCDEFGH
                // w:                  <- 0
                set_coarse_y(&mut self.temporary_vram, scroll_value >> 3);
                set_fine_y(&mut self.temporary_vram, scroll_value & 0b111);
                self.write_toggle = false
            }
            false => {
                // if w is 0
                // t: ....... ...ABCDE <- d: ABCDE...
                // x:              FGH <- d: .....FGH
                // w:                  <- 1
                set_coarse_x(&mut self.temporary_vram, scroll_value >> 3);
                self.fine_x = scroll_value & 0b111;
                self.write_toggle = true;
            }
        }
    }

    pub fn write_address(&mut self, address: u8) {
        match self.write_toggle {
            false => {
                // t: .CDEFGH ........ <- d: ..CDEFGH
                self.temporary_vram &= 0x00ff;
                self.temporary_vram |= ((address & 0x3f) as u16) << 8;
                //   <unused>     <- d: AB......
                // t: Z...... ........ <- 0 (bit Z is cleared)
                self.temporary_vram &= 0x3fff;
                // w:                  <- 1
                self.write_toggle = true
            }
            true => {
                // t: ....... ABCDEFGH <- d: ABCDEFGH
                // v: <...all bits...> <- t: <...all bits...>
                // w:                  <- 0
                self.temporary_vram = (self.temporary_vram & 0xff00) | (address as u16);
                self.current_vram = self.temporary_vram;
                self.write_toggle = false
            }
        }
    }

    pub fn get_address_u16(&self) -> u16 {
        self.current_vram
    }

    pub fn increment_v(&mut self, incr: u8) {
        self.current_vram = self.current_vram.wrapping_add(incr as u16);
    }

    pub fn get_x_scroll(&self) -> u8 {
        (get_coarse_x(self.current_vram) << 3) | self.fine_x
    }

    pub fn get_y_scroll(&self) -> u8 {
        (get_coarse_y(self.current_vram) << 3) | get_fine_y(self.current_vram)
    }

    pub fn increase_coarse_x(&mut self) {
        // increases the coarse x, and switched nametable x if needed
        let coarse = get_coarse_x(self.current_vram);
        if coarse == 31 {
            // we need to switch horizontal nametable
            let current_nametable = get_nametable(self.current_vram);
            set_nametable(&mut self.current_vram, current_nametable ^ 1);
            set_coarse_x(&mut self.current_vram, 0);
        } else {
            set_coarse_x(&mut self.current_vram, coarse + 1);
        }
    }

    pub fn increase_y(&mut self) {
        // increase y, with overflow to coarse y and wrapping around the nametable
        let fine_y = get_fine_y(self.current_vram);
        if fine_y < 7 {
            set_fine_y(&mut self.current_vram, fine_y + 1);
            return;
        }
        // else, we must increase coarse y
        set_fine_y(&mut self.current_vram, 0);
        let coarse_y = get_coarse_y(self.current_vram);
        if coarse_y == 29 {
            // last row, switch y nametable
            let nametable = get_nametable(self.current_vram);
            set_nametable(&mut self.current_vram, nametable ^ 2);
            set_coarse_y(&mut self.current_vram, 0);
        } else if coarse_y == 31 {
            // edge case - reset coarse y but don't switch nametable
            set_coarse_y(&mut self.current_vram, 0);
        } else {
            set_coarse_y(&mut self.current_vram, coarse_y + 1);
        }
    }

    pub fn copy_t_to_v(&mut self) {
        self.current_vram = self.temporary_vram
    }
}
