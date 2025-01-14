use crate::bus::cartridge::Mirroring;
use crate::bus::Bus;

pub const PPU_CHR_ROM_START: u16 = 0x0000;
pub const PPU_CHR_ROM_END: u16 = 0x1FFF;
pub const PPU_NAMETABLE_START: u16 = 0x2000;
pub const PPU_NAMETABLE_SIZE: u16 = 0x400;
pub const PPU_NAMETABLE_END: u16 = 0x2FFF;
pub const PPU_UNUSED_SEG_START: u16 = 0x3000;
pub const PPU_UNUSED_SEG_END: u16 = 0x3EFF;
pub const PPU_PALETTE_START: u16 = 0x3F00;
pub const PPU_PALETTE_SIZE: u16 = 32;
pub const PPU_PALETTE_END: u16 = 0x3FFF;

#[derive(Debug, Eq, PartialEq)]
pub struct PPUMemory {
    pub palette_table: [u8; 32],
    pub vram: [u8; (PPU_NAMETABLE_END - PPU_NAMETABLE_START + 1) as usize],
    pub oam_data: [u8; 256],
}

impl PPUMemory {
    pub fn new() -> Self {
        Self {
            palette_table: [0; 32],
            vram: [0; (PPU_NAMETABLE_END - PPU_NAMETABLE_START + 1) as usize],
            oam_data: [0; 256],
        }
    }
}

impl Bus {
    pub fn read_vram(&self, addr_in_vram: usize) -> u8 {
        // this function get addr in vram (0x0000 to 0x0fff)
        // and returns the "true" value of the vram in this address, after mirroring
        let addr_in_vram = addr_in_vram % (4 * PPU_NAMETABLE_SIZE as usize);
        let addr_in_ppu_mem = addr_in_vram as u16 + PPU_NAMETABLE_START;
        let mirrored_addr_in_ppu_mem = self.mirror_vram_address(addr_in_ppu_mem);
        self.ppu_memory.vram[mirrored_addr_in_ppu_mem as usize - PPU_NAMETABLE_START as usize]
    }

    fn mirror_vram_address(&self, vram_address: u16) -> u16 {
        // mirrors the vram address to the "canonical" address based on the mirroring mode
        // as I understand it: the possible addresses are from 0x2000 to 0x2fff, and are split to 4 "chunks" (name-tables)

        let address_offset = vram_address - PPU_NAMETABLE_START;
        let table_number = address_offset / PPU_NAMETABLE_SIZE;
        match self.cartridge.screen_mirroring {
            Mirroring::Horizontal => {
                // [A] [A]
                // [B] [B]
                match table_number {
                    0 => vram_address,
                    1 => vram_address - PPU_NAMETABLE_SIZE,
                    2 => vram_address,
                    3 => vram_address - PPU_NAMETABLE_SIZE,
                    _ => panic!("Somehow messed up vram mirroring. Asked for address {vram_address} which resolved in table {table_number}")
                }
            }
            Mirroring::Vertical => {
                // [A] [B]
                // [A] [B]
                match table_number {
                    0 => vram_address,
                    1 => vram_address,
                    2 => vram_address - 2 * PPU_NAMETABLE_SIZE,
                    3 => vram_address - 2 * PPU_NAMETABLE_SIZE,
                    _ => panic!("Somehow messed up vram mirroring. Asked for address {vram_address} which resolved in table {table_number}")
                }
            }
            Mirroring::FourScreen => {
                // maybe this doesn't have mirroring at all? Seems like a very rare mode
                vram_address
            }
            Mirroring::Unloaded => {
                panic!("Trying to access ppu address when the mirroring is unloaded")
            }
        }
    }

    pub fn copy_from_ram_to_oam(&mut self) {
        // usize should mean u16, since this is the address space in the nes
        let current_oam_addr: usize =
            self.ppu_registers.oam_addr_register.read_current_value() as usize;
        let ram_location: usize = ((self.oam_dma_page as usize) << 8) + current_oam_addr;
        self.ppu_memory.oam_data[current_oam_addr] = self.cpu_ram[ram_location];
        self.ppu_registers.oam_addr_register.increase(1);
        self.cpu_idle_cycles += 2; // this action takes two cycles
        self.number_of_copies_in_current_oam_dma =
            self.number_of_copies_in_current_oam_dma.wrapping_add(1);
    }

    fn convert_ppu_address_to_actual_address(&mut self, address: u16) -> &mut u8 {
        match address {
            PPU_CHR_ROM_START..=PPU_CHR_ROM_END => &mut self.cartridge.chr_rom[address as usize],
            PPU_NAMETABLE_START..=PPU_NAMETABLE_END => {
                let canonical_address = self.mirror_vram_address(address) as usize;
                &mut self.ppu_memory.vram[canonical_address - PPU_NAMETABLE_START as usize]
            }
            PPU_UNUSED_SEG_START..=PPU_UNUSED_SEG_END => &mut self.ppu_memory.vram[0], // panic!(
            //"addr space 0x3000..0x3eff is not expected to be used, requested = {} ",
            //address
            //),
            PPU_PALETTE_START..=PPU_PALETTE_END => {
                //Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C
                let index_in_palette_table = match address {
                    0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                        (address - 0x10 - PPU_PALETTE_START) % PPU_PALETTE_SIZE
                    }
                    _ => (address - PPU_PALETTE_START) % PPU_PALETTE_SIZE,
                };
                &mut self.ppu_memory.palette_table[index_in_palette_table as usize]
            }
            _ => panic!("unexpected access to mirrored space {}", address),
        }
    }

    fn read_ppu_data_register_from_address(&mut self, address: u16) -> u8 {
        // we need to return the value of the current buffer, and then update the buffer
        let result = self.ppu_registers.data_register.read_current_value();

        let new_result_pointer = self.convert_ppu_address_to_actual_address(address);
        let new_result_value = *new_result_pointer;
        let data_register = &mut self.ppu_registers.data_register;
        data_register.update_current_value(new_result_value);

        // only when reading from the palette we get the result imminently.
        // I think the result is also inserted to the buffer as normal
        if PPU_PALETTE_START <= address && address <= PPU_PALETTE_END {
            self.ppu_registers.data_register.read_current_value()
        } else {
            result
        }
    }

    //noinspection RsNonExhaustiveMatch
    pub fn read_ppu_memory(&mut self, canonical_address: u16) -> u8 {
        match canonical_address {
            0x00..=0x01fff => panic!("Error: address {canonical_address} is not in range of ppu registers"),
            0x2000 => panic!("register CTRL is write only1"), // PPUCTRL
            0x2001 => self.ppu_registers.mask_register.read(), //PPUMASK
            0x2002 => { //PPUSTATUS
                // reading from status register has the strange attribute of resetting the latch w
                self.ppu_registers.reset_latch();
                let status_register = self.ppu_registers.status_register.read();
                self.ppu_registers.status_register.set_vblank_status(false); // clear vblank
                status_register
            },
            0x2003 => panic!("register OAM address is write only"), //OAMADDR, write only
            0x2004 => todo!(), //OAMDATA
            0x2005 => 0, // PPUSCRL, should be write-only
            0x2006 => panic!("addr register is read only"), //PPUADDR
            0x2007 => { //PPUDATA
                let address = self.ppu_registers.get_address_u16();
                self.ppu_registers.increase_address(self.ppu_registers.control_register.get_vram_address_inc());
                self.read_ppu_data_register_from_address(address)
            },
            0x2008..=0x3FFF => panic!("Address {canonical_address} is ppu register but mirrored - the mirror logic should have been in the caller"),
            0x4000..=0xFFFF =>panic!("Error: address {canonical_address} is not in range of ppu registers"),
        }
    }

    //noinspection RsNonExhaustiveMatch
    pub fn write_ppu_memory(&mut self, address: u16, value: u8) {
        match address {
            0x00..=0x01fff => panic!("Error: address {address} is not in range of ppu registers"),
            0x2000 =>            {// PPUCTRL
                self.ppu_registers.write_control(value);
            },
            0x2001 => self.ppu_registers.mask_register.write_byte(value), //PPUMASK
            0x2002 => panic!("Should not write to read-only status register PPUSTATUS at 0x2002"), //PPUSTATUS
            0x2003 => self.ppu_registers.oam_addr_register.update_current_value(value), //OAMADDR
            0x2004 => todo!(), //OAMDATA
            0x2005 => self.ppu_registers.write_to_scroll(value), // PPUSCRL
            0x2006 => self.ppu_registers.write_to_addr_reg(value), //PPUADDR
            0x2007 => {
                let address_in_ppu = self.ppu_registers.get_address_u16();
                self.ppu_registers.increase_address(self.ppu_registers.control_register.get_vram_address_inc());
                let pointer_to_address = self.convert_ppu_address_to_actual_address(address_in_ppu);
                *pointer_to_address = value;
            }, //PPUDATA
            0x2008..=0x3FFF => panic!("Address {address} is ppu register but mirrored - the mirror logic should have been in the caller"),
            0x4000..=0xFFFF =>panic!("Error: address {address} is not in range of ppu registers"),
        }
    }
}
