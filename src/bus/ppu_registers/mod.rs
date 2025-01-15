use crate::bus::ppu_memory::PPU_NAMETABLE_SIZE;
use crate::bus::ppu_registers::control_register::PPUControlRegister;
use crate::bus::ppu_registers::data_register::PPUDataReg;
use crate::bus::ppu_registers::internal_registers::InternalPPURegisters;
use crate::bus::ppu_registers::mask_register::PPUMaskRegister;
use crate::bus::ppu_registers::oam_address_register::OAMAdressRegister;
use crate::bus::ppu_registers::status_register::PPUStatusRegister;

pub mod address_register;
pub mod control_register;
mod data_register;
mod internal_registers;
mod mask_register;
mod oam_address_register;
mod status_register;

#[derive(Debug, Eq, PartialEq)]
pub struct PPURegisters {
    pub control_register: PPUControlRegister,
    pub mask_register: PPUMaskRegister,
    pub status_register: PPUStatusRegister,
    pub oam_addr_register: OAMAdressRegister,
    pub data_register: PPUDataReg,
    internal_registers: InternalPPURegisters, // combination of PPUSCROLL and PPUADDR
}

impl PPURegisters {
    pub fn new() -> Self {
        Self {
            control_register: PPUControlRegister::new(),
            mask_register: PPUMaskRegister::new(),
            status_register: PPUStatusRegister::new(),
            oam_addr_register: OAMAdressRegister::new(),
            data_register: PPUDataReg::new(),
            internal_registers: InternalPPURegisters::new(),
        }
    }

    pub fn reset_latch(&mut self) {
        self.internal_registers.reset_toggle()
    }

    pub fn set_latch(&mut self) {
        self.internal_registers.set_toggle()
    }

    pub fn write_to_addr_reg(&mut self, value: u8) {
        self.internal_registers.write_address(value);
    }

    pub fn write_to_scroll(&mut self, value: u8) {
        self.internal_registers.write_scroll(value);
    }

    pub fn write_control(&mut self, value: u8) {
        self.control_register.write_byte(value);
        self.internal_registers.write_control(value);
    }

    pub fn get_address_u16(&self) -> u16 {
        self.internal_registers.get_address_u16()
    }

    pub fn increase_address(&mut self, incr: u8) {
        self.internal_registers.increment_v(incr);
    }

    pub fn get_coarse_x(&self) -> u8 {
        self.internal_registers.get_coarse_x()
    }

    pub fn get_fine_x(&self) -> u8 {
        self.internal_registers.get_fine_x()
    }

    pub fn get_x_scroll(&self) -> u8 {
        self.internal_registers.get_x_scroll()
    }

    pub fn get_coarse_y(&self) -> u8 {
        self.internal_registers.get_coarse_y()
    }

    pub fn get_fine_y(&self) -> u8 {
        self.internal_registers.get_fine_y()
    }

    pub fn get_y_scroll(&self) -> u8 {
        self.internal_registers.get_y_scroll()
    }

    pub fn get_nametable_base_index(&self) -> u16 {
        self.internal_registers.get_nametable_index() as u16 * PPU_NAMETABLE_SIZE
    }

    pub fn copy_t_to_v(&mut self) {
        self.internal_registers.copy_t_to_v();
    }
}
