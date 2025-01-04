#[cfg(test)]
mod counting_page_cross_for_cycles_test;
#[cfg(test)]
mod mem_tests;

use crate::bus::memory::Mem;
use crate::bus_mut;
use crate::cpu::CPU;

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,

    //TODO: remove these later
    Accumulator,
    Relative,
    Indirect,
}

pub fn check_if_on_different_pages(addr1: u16, addr2: u16) -> bool {
    (addr1 & 0xFF00) != (addr2 & 0xFF00)
}

impl<'a> CPU<'a> {
    pub fn read_memory(&mut self, addr: u16) -> u8 {
        bus_mut!(self).read_memory(addr)
    }

    pub fn read_memory_2_bytes(&mut self, addr: u16) -> u16 {
        bus_mut!(self).read_memory_2_bytes(addr)
    }

    pub fn read_memory_2_bytes_without_page_cross(&mut self, addr: u16) -> u16 {
        // this function is like read 2 bytes, but does not cross the page boundary
        // so if we ask for 12ff it brings the bytes from 12ff and 1200, and not 12ff and 1300
        let low_byte_location = addr;
        let high_byte_location = match addr & 0xff {
            0xff => addr - 0xff,
            _ => addr.wrapping_add(1),
        };
        let low = bus_mut!(self).read_memory(low_byte_location) as u16;
        let high = bus_mut!(self).read_memory(high_byte_location) as u16;
        (high << 8) | low
    }

    pub fn write_memory(&mut self, addr: u16, data: u8) {
        bus_mut!(self).write_memory(addr, data);
    }

    pub fn convert_mode_to_val(&mut self, mode: AddressingMode) -> u8 {
        let operand_memory_address = self.convert_mode_to_operand_mem_address(mode);
        self.read_memory(operand_memory_address)
    }

    pub fn convert_mode_to_operand_mem_address(&mut self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.read_memory(self.program_counter) as u16,
            AddressingMode::ZeroPage_X => {
                (self.read_memory(self.program_counter) as u16 + self.register_x as u16) & 0x00FF
            }

            AddressingMode::ZeroPage_Y => self
                .read_memory(self.program_counter)
                .wrapping_add(self.register_y) as u16,

            AddressingMode::Relative => {
                let unsigned_offset = self.read_memory(self.program_counter);
                let mut signed_offset = unsigned_offset as i16;
                if signed_offset >= 0b1000_0000 {
                    signed_offset = signed_offset - 0b1_0000_0000;
                }
                ((self.program_counter + 2) as i32 + signed_offset as i32) as u16
            }
            AddressingMode::Absolute => self.read_memory_2_bytes(self.program_counter),
            AddressingMode::Absolute_X => self
                .read_memory_2_bytes(self.program_counter)
                .wrapping_add(self.register_x as u16),
            AddressingMode::Absolute_Y => self
                .read_memory_2_bytes(self.program_counter)
                .wrapping_add(self.register_y as u16),
            AddressingMode::Indirect => {
                let let_intermediate_memory_value = self.read_memory_2_bytes(self.program_counter);
                self.read_memory_2_bytes_without_page_cross(let_intermediate_memory_value)
            }
            AddressingMode::Indirect_X => {
                let zero_page_location: u8 = self
                    .register_x
                    .wrapping_add(self.read_memory(self.program_counter));
                self.read_memory_2_bytes_without_page_cross(zero_page_location as u16)
            }
            AddressingMode::Indirect_Y => {
                let zero_page_mem_location = self.read_memory(self.program_counter);
                self.read_memory_2_bytes_without_page_cross(zero_page_mem_location as u16)
                    .wrapping_add(self.register_y as u16)
            }
            AddressingMode::Accumulator => 0,
            AddressingMode::NoneAddressing => 0,
        }
    }

    pub fn detect_page_cross(&mut self, addressing_mode: AddressingMode) -> bool {
        // check if page cross happened while fetching the value
        match addressing_mode {
            AddressingMode::Absolute_X => check_if_on_different_pages(
                self.read_memory_2_bytes(self.program_counter),
                self.convert_mode_to_operand_mem_address(addressing_mode),
            ),
            AddressingMode::Absolute_Y => check_if_on_different_pages(
                self.read_memory_2_bytes(self.program_counter),
                self.convert_mode_to_operand_mem_address(addressing_mode),
            ),
            AddressingMode::Indirect_Y => {
                let zero_page_mem_location = self.read_memory(self.program_counter);
                let address_before_adding_y =
                    self.read_memory_2_bytes_without_page_cross(zero_page_mem_location as u16);
                check_if_on_different_pages(
                    address_before_adding_y,
                    self.convert_mode_to_operand_mem_address(addressing_mode),
                )
            }
            _ => false,
        }
    }
}
