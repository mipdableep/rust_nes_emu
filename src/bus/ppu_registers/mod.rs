use crate::bus::ppu_registers::address_register::PPUAddressReg;
use crate::bus::ppu_registers::control_register::PPUControlRegister;
use crate::bus::ppu_registers::data_register::PPUDataReg;
use crate::bus::ppu_registers::mask_register::PPUMaskRegister;
use crate::bus::ppu_registers::oam_address_register::OAMAdressRegister;
use crate::bus::ppu_registers::scroll_register::PPUScrollReg;
use crate::bus::ppu_registers::status_register::PPUStatusRegister;
use crate::ppu::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub mod address_register;
pub mod control_register;
mod data_register;
mod mask_register;
mod oam_address_register;
mod scroll_register;
mod status_register;

#[derive(Debug, Eq, PartialEq)]
pub struct PPURegisters {
    pub control_register: PPUControlRegister,
    pub mask_register: PPUMaskRegister,
    pub status_register: PPUStatusRegister,
    pub oam_addr_register: OAMAdressRegister,
    pub address_register: PPUAddressReg,
    pub data_register: PPUDataReg,
    pub scroll_register: PPUScrollReg,
    internal_latch: bool, // scroll and addr use the same latch
}

impl PPURegisters {
    pub fn new() -> Self {
        Self {
            control_register: PPUControlRegister::new(),
            mask_register: PPUMaskRegister::new(),
            status_register: PPUStatusRegister::new(),
            oam_addr_register: OAMAdressRegister::new(),
            address_register: PPUAddressReg::new(),
            data_register: PPUDataReg::new(),
            scroll_register: PPUScrollReg::new(),
            internal_latch: true,
        }
    }

    pub fn reset_latch(&mut self) {
        self.internal_latch = true;
    }

    pub fn write_to_addr_reg(&mut self, value: u8) {
        self.address_register
            .write_byte(value, &mut self.internal_latch);

        // Writes here overwrite the current nametable bits in ppu_ctrl (or at least overwrite bits in an
        // internal latch that is equivalent to this); see [7].  Some games, e.g. SMB, rely on this behaviour
        let new_control_value =
            self.control_register.read() & 0b11111100 | (value & 0b00001100 >> 2);
        self.control_register.write_byte(new_control_value);
    }

    pub fn write_to_scroll(&mut self, value: u8) {
        self.scroll_register
            .write_byte(value, &mut self.internal_latch);
    }

    pub fn get_abs_x_offset(&self) -> usize {
        self.scroll_register.get_x_scroll() as usize
            + self.control_register.get_nametable_x() * SCREEN_WIDTH
    }

    pub fn get_abs_y_offset(&self) -> usize {
        self.scroll_register.get_y_scroll() as usize
            + self.control_register.get_nametable_y() * SCREEN_HEIGHT
    }

    pub fn get_tile_background_tile_bank(&self) -> u16 {
        self.control_register.get_background_sprite_address()
    }

    pub fn get_sprite_tile_bank(&self) -> u16 {
        self.control_register.get_sprite_pattern_address()
    }
}
