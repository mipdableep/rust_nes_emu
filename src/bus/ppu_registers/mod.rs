use crate::bus::ppu_registers::address_register::PPUAddressReg;
use crate::bus::ppu_registers::control_register::PPUControlRegister;
use crate::bus::ppu_registers::data_register::PPUDataReg;

pub mod address_register;
pub mod control_register;
mod data_register;

#[derive(Debug, Eq, PartialEq)]
pub struct PPURegisters {
    pub address_register: PPUAddressReg,
    pub control_register: PPUControlRegister,
    pub data_register: PPUDataReg,
}

impl PPURegisters {
    pub fn new() -> Self {
        Self {
            address_register: PPUAddressReg::new(),
            control_register: PPUControlRegister::new(),
            data_register: PPUDataReg::new(),
        }
    }
}
