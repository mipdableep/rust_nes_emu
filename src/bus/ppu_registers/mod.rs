use crate::bus::ppu_registers::address_register::PPUAddressReg;
use crate::bus::ppu_registers::control_register::PPUControlRegister;
use crate::bus::ppu_registers::data_register::PPUDataReg;
use crate::bus::ppu_registers::mask_register::PPUMaskRegister;
use crate::bus::ppu_registers::oam_address_register::OAMAdressRegister;
use crate::bus::ppu_registers::scroll_register::PPUScrollReg;
use crate::bus::ppu_registers::status_register::PPUStatusRegister;

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
    }
}
