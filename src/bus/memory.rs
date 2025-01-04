use crate::bus::memory_mapping_constants::*;
use crate::bus::Bus;

pub trait Mem {
    fn write_memory(&mut self, addr: u16, data: u8);
    fn read_memory_2_bytes(&mut self, addr: u16) -> u16;
    fn read_memory(&mut self, addr: u16) -> u8;
}

impl Mem for Bus {
    //noinspection RsNonExhaustiveMatch
    fn write_memory(&mut self, addr: u16, data: u8) {
        match addr {
            CPU_RAM_MEM_START..=CPU_RAM_MEM_END => {
                // manage mirroring
                let offset_from_start = addr - CPU_RAM_MEM_START;
                let canonical_offset_from_start = offset_from_start % CPU_RAM_MEM_UNIQUE_SIZE;
                let canonical_address = CPU_RAM_MEM_START + canonical_offset_from_start;
                self.cpu_ram[(canonical_address - CPU_RAM_MEM_START) as usize] = data;
            }
            PPU_REGISTERS_START..=PPU_REGISTERS_END => {
                // manage mirroring
                let offset_from_start = addr - PPU_REGISTERS_START;
                let canonical_offset_from_start = offset_from_start % PPU_REGISTERS_UNIQUE_SIZE;
                let canonical_address = PPU_REGISTERS_START + canonical_offset_from_start;
                self.write_ppu_memory(canonical_address, data);
            }
            IO_AND_AUDIO_REGISTERS_START..=IO_AND_AUDIO_REGISTERS_END => {
                if addr == OAM_DMA {
                    // the oam dma copies a full page to the ppu
                    // for starters, it will take 1 cycles (should take 1/2),
                    // and will halt the cpu for 512 cycles - 256 (full page) of 2 cycle copy

                    // cartridge.chr_rom[7 * 16..8 * 16].copy_from_slice(&our_tile);
                    let page_start: usize = (data as usize) << 8;
                    self.ppu_memory
                        .oam_data
                        .copy_from_slice(&self.cpu_ram[page_start..page_start + 256]);
                }
                self.io_and_audio_registers[(addr - IO_AND_AUDIO_REGISTERS_START) as usize] = data;
            }
            UNMAPPED_SEG_START..=UNMAPPED_SEG_END => {
                self.unmapped_seg[(addr - UNMAPPED_SEG_START) as usize] = data;
            }
            PRG_RAM_START..=PRG_RAM_END => {
                self.prg_ram[(addr - PRG_RAM_START) as usize] = data;
            }
            PRG_ROM_START..=PRG_ROM_END => {
                panic!("trying to write to the ROM in address {addr}");
            }
        }
    }

    fn read_memory_2_bytes(&mut self, addr: u16) -> u16 {
        let low = self.read_memory(addr) as u16;
        let high = self.read_memory(addr + 1) as u16;
        (high << 8) | (low as u16)
    }

    //noinspection RsNonExhaustiveMatch
    fn read_memory(&mut self, addr: u16) -> u8 {
        match addr {
            CPU_RAM_MEM_START..=CPU_RAM_MEM_END => {
                // manage mirroring
                let offset_from_start = addr - CPU_RAM_MEM_START;
                let canonical_offset_from_start = offset_from_start % CPU_RAM_MEM_UNIQUE_SIZE;
                let canonical_address = CPU_RAM_MEM_START + canonical_offset_from_start;
                self.cpu_ram[(canonical_address - CPU_RAM_MEM_START) as usize]
            }
            PPU_REGISTERS_START..=PPU_REGISTERS_END => {
                // manage mirroring
                let offset_from_start = addr - PPU_REGISTERS_START;
                let canonical_offset_from_start = offset_from_start % PPU_REGISTERS_UNIQUE_SIZE;
                let canonical_address = PPU_REGISTERS_START + canonical_offset_from_start;
                self.read_ppu_memory(canonical_address)
            }
            IO_AND_AUDIO_REGISTERS_START..=IO_AND_AUDIO_REGISTERS_END => {
                self.io_and_audio_registers[(addr - IO_AND_AUDIO_REGISTERS_START) as usize]
            }
            UNMAPPED_SEG_START..=UNMAPPED_SEG_END => {
                self.unmapped_seg[(addr - UNMAPPED_SEG_START) as usize]
            }
            PRG_RAM_START..=PRG_RAM_END => self.prg_ram[(addr - PRG_RAM_START) as usize],
            PRG_ROM_START..=PRG_ROM_END => self.cartridge.read_prg_rom(addr - PRG_ROM_START),
        }
    }
}
