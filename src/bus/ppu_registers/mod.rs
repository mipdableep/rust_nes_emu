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

    //noinspection RsNonExhaustiveMatch
    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x00..=0x01fff => panic!("Error: address {address} is not in range of ppu registers"),
            0x2000 => self.control_register.write_byte(value), // PPUCTRL
            0x2001 => todo!(), //PPUMASK
            0x2002 => todo!(), //PPUSTATUS
            0x2003 => todo!(), //OAMADDR
            0x2004 => todo!(), //OAMDATA
            0x2005 => todo!(), // PPUSCRL
            0x2006 => self.address_register.write_byte(value), //PPUADDR
            0x2007 => todo!(), //PPUDATA
            0x2008..=0x3FFF => panic!("Address {address} is ppu register but mirrored - the mirror logic should have been in the caller"),
            0x4000..=0xFFFF =>panic!("Error: address {address} is not in range of ppu registers"),
        }
    }
}
