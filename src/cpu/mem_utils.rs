mod mem_tests;

use crate::cpu::CPU;

#[derive(Debug)]
#[allow(non_camel_case_types, dead_code)]
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

#[allow(dead_code)]
impl CPU {
    pub fn read_memory(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn read_memory_2_bytes(&self, addr: u16) -> u16 {
        let low = self.read_memory(addr) as u16;
        let high = self.read_memory(addr + 1) as u16;
        (high << 8) | (low as u16)
    }

    pub fn read_memory_2_bytes_with_overflow(&self, addr: u16) -> u16 {
        let low = self.read_memory(addr) as u16;
        let high = self.read_memory(addr.wrapping_add(1)) as u16;
        (high << 8) | (low as u16)
    }

    pub fn write_memory(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn convert_mode_to_val(&self, mode: AddressingMode) -> u8 {
        self.read_memory(self.convert_mode_to_operand_mem_address(mode))
    }

    pub fn convert_mode_to_operand_mem_address(&self, mode: AddressingMode) -> u16 {
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
            AddressingMode::Absolute_X => {
                self.read_memory_2_bytes((self.program_counter) as u16)
                    .wrapping_add(self.register_x as u16) as u16
            }
            AddressingMode::Absolute_Y => {
                self.read_memory_2_bytes((self.program_counter) as u16)
                    .wrapping_add(self.register_y as u16) as u16
            }
            AddressingMode::Indirect => {
                self.read_memory_2_bytes(self.read_memory_2_bytes(self.program_counter))
            }
            AddressingMode::Indirect_X => {
                let zero_page_location: u8 = self
                    .register_x
                    .wrapping_add(self.read_memory(self.program_counter));
                self.read_memory_2_bytes_with_overflow(zero_page_location as u16)
            }
            AddressingMode::Indirect_Y => {
                let mem_location = self
                    .read_memory_2_bytes((self.program_counter) as u16)
                    .wrapping_add(self.register_y as u16);
                self.read_memory_2_bytes_with_overflow(mem_location)
            }
            AddressingMode::Accumulator => 0,
            AddressingMode::NoneAddressing => {
                panic!("Unimplemented mode {:?}", mode)
            }
        }
    }
}
