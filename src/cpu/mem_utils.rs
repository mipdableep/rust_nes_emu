use crate::cpu::CPU;

#[derive(Debug)]
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

impl CPU {
    fn read_memory(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn read_memory_2_bytes(&self, addr: u16) -> u16 {
        let low = self.mem_read(addr) as u16;
        let high = self.mem_read(addr + 1) as u16;
        (high << 8) | (low as u16)
    }

    fn read_memory_2_bytes_with_overflow(&self, addr: u16) -> u16 {
        let low = self.mem_read(addr) as u16;
        let high = self.mem_read(addr.wrapping_add(1)) as u16;
        (high << 8) | (low as u16)
    }
