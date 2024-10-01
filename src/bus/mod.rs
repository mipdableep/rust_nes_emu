mod cartrige;
#[cfg(test)]
pub mod mem_tests;
pub(crate) mod memory;

use memory_mapping_constants::*;

pub(crate) mod memory_mapping_constants {
    pub const CPU_RAM_MEM_START: u16 = 0x0000;
    pub const CPU_RAM_MEM_UNIQUE_SIZE: u16 = 0x0800;
    pub const CPU_RAM_MEM_END: u16 = 0x1FFF;
    pub const PPU_REGISTERS_START: u16 = 0x2000;
    pub const PPU_REGISTERS_UNIQUE_SIZE: u16 = 0x0008;
    pub const PPU_REGISTERS_END: u16 = 0x3FFF;
    pub const IO_AND_AUDIO_REGISTERS_START: u16 = 0x4000;
    pub const IO_AND_AUDIO_REGISTERS_END: u16 = 0x401F;
    pub const UNMAPPED_SEG_START: u16 = 0x4020;
    pub const UNMAPPED_SEG_END: u16 = 0x5FFF;
    pub const PRG_RAM_START: u16 = 0x6000;
    pub const PRG_RAM_END: u16 = 0x7FFF;
    pub const PRG_ROM_START: u16 = 0x8000;
    pub const PRG_ROM_END: u16 = 0xFFFF;
}

#[derive(Debug, Eq, PartialEq)]
pub struct Bus {
    cpu_ram: [u8; CPU_RAM_MEM_UNIQUE_SIZE as usize],
    ppu_registers: [u8; PPU_REGISTERS_UNIQUE_SIZE as usize],
    io_and_audio_registers:
        [u8; (IO_AND_AUDIO_REGISTERS_END - IO_AND_AUDIO_REGISTERS_START + 1) as usize],
    unmapped_seg: [u8; (UNMAPPED_SEG_END - UNMAPPED_SEG_START + 1) as usize],
    prg_ram: [u8; (PRG_RAM_END - PRG_RAM_START + 1) as usize],
    pub prg_rom: [u8; (PRG_ROM_END - PRG_ROM_START + 1) as usize],
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            cpu_ram: [0; CPU_RAM_MEM_UNIQUE_SIZE as usize],
            ppu_registers: [0; PPU_REGISTERS_UNIQUE_SIZE as usize],
            io_and_audio_registers: [0; (IO_AND_AUDIO_REGISTERS_END - IO_AND_AUDIO_REGISTERS_START
                + 1) as usize],
            unmapped_seg: [0; (UNMAPPED_SEG_END - UNMAPPED_SEG_START + 1) as usize],
            prg_ram: [0; (PRG_RAM_END - PRG_RAM_START + 1) as usize],
            prg_rom: [0; (PRG_ROM_END - PRG_ROM_START + 1) as usize],
        }
    }
}
