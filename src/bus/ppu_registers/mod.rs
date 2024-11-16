use crate::bus::ppu_registers::address_register::PPUAddressReg;
use crate::bus::ppu_registers::control_register::PPUControlRegister;
use crate::bus::ppu_registers::data_register::PPUDataReg;
use crate::bus::ppu_registers::mask_register::PPUMaskRegister;
use crate::bus::ppu_registers::status_register::PPUStatusRegister;

pub mod address_register;
pub mod control_register;
mod data_register;
mod mask_register;
mod status_register;

#[derive(Debug, Eq, PartialEq)]
pub struct PPURegisters {
    pub control_register: PPUControlRegister,
    pub mask_register: PPUMaskRegister,
    pub status_register: PPUStatusRegister,
    pub address_register: PPUAddressReg,
    pub data_register: PPUDataReg,
}

impl PPURegisters {
    pub fn new() -> Self {
        Self {
            control_register: PPUControlRegister::new(),
            mask_register: PPUMaskRegister::new(),
            status_register: PPUStatusRegister::new(),
            address_register: PPUAddressReg::new(),
            data_register: PPUDataReg::new(),
        }
    }
}
