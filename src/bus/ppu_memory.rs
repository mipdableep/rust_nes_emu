use crate::bus::Bus;

#[derive(Debug, Eq, PartialEq)]
pub struct PPUMemory {
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],
}

impl PPUMemory {
    pub fn new() -> Self {
        Self {
            palette_table: [0; 32],
            vram: [0; 2048],
            oam_data: [0; 256],
        }
    }
}

impl Bus {
    //noinspection RsNonExhaustiveMatch
    pub fn read_ppu_memory(&mut self, canonical_address: u16) -> u8 {
        match canonical_address {
            0x00..=0x01fff => panic!("Error: address {canonical_address} is not in range of ppu registers"),
            0x2000 => self.ppu_registers.control_register.read(), // PPUCTRL
            0x2001 => todo!(), //PPUMASK
            0x2002 => todo!(), //PPUSTATUS
            0x2003 => todo!(), //OAMADDR
            0x2004 => todo!(), //OAMDATA
            0x2005 => todo!(), // PPUSCRL
            0x2006 => self.ppu_registers.address_register.read(), //PPUADDR
            0x2007 => { //PPUDATA
                // we need to return the value of the current buffer, and then update the buffer
                let result = self.ppu_registers.data_register.read_current_value();
                let address = self.ppu_registers.address_register.get_address_as_u16();

                match address {
                    0..=0x1fff => {
                        let data_register = &mut self.ppu_registers.data_register;
                        data_register.update_current_value(self.cartridge.chr_rom[address as usize]);
                    },
                    0x2000..=0x2fff => todo!("read from RAM"),
                    0x3000..=0x3eff => panic!("addr space 0x3000..0x3eff is not expected to be used, requested = {} ", address),
                    0x3f00..=0x3fff =>
                        {
                            return self.ppu_memory.palette_table[(address-0x3f00) as usize];
                        }
                    _ => panic!("unexpected access to mirrored space {}", address),
                }

                result
            },
            0x2008..=0x3FFF => panic!("Address {canonical_address} is ppu register but mirrored - the mirror logic should have been in the caller"),
            0x4000..=0xFFFF =>panic!("Error: address {canonical_address} is not in range of ppu registers"),

        }
    }
}
