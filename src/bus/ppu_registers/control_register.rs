use std::ops::Deref;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PPUControlRegister(u8);

// 7  bit  0
// ---- ----
// VPHB SINN
// |||| ||||
// |||| ||++- Base nametable address
// |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
// |||| |+--- VRAM address increment per CPU read/write of PPUDATA
// |||| |     (0: add 1, going across; 1: add 32, going down)
// |||| +---- Sprite pattern table address for 8x8 sprites
// ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
// |||+------ Background pattern table address (0: $0000; 1: $1000)
// ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels – see PPU OAM#Byte 1)
// |+-------- PPU master/slave select
// |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
// +--------- Vblank NMI enable (0: off, 1: on)

impl Deref for PPUControlRegister {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PPUControlRegister {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn read(&self) -> u8 {
        self.0
    }

    pub fn write_byte(&mut self, value: u8) {
        self.0 = value;
    }

    fn get_bit(&self, bit_location: u8) -> bool {
        // a function to return the value of the bit in bit location
        if bit_location > 7 {
            panic!("Error: Trying to access bit in location {bit_location} in the PPU control register, which does not exists");
        }
        (self.0 >> bit_location) & 1 == 1
    }

    pub fn get_vblank_nmi(&self) -> bool {
        self.get_bit(7)
    }
    // lazy add the wrappers to other bits: add only when needed
    pub fn get_vram_address_inc(&self) -> u8 {
        match self.get_bit(2) {
            true => 32,
            false => 1,
        }
    }

    pub fn get_sprite_pattern_address(&self) -> u16 {
        match self.get_bit(3) {
            true => 0x1000,
            false => 0x0000,
        }
    }

    pub fn get_background_sprite_address(&self) -> u16 {
        match self.get_bit(4) {
            true => 0x1000,
            false => 0x0000,
        }
    }

    pub fn get_nametable_offset(&self) -> u16 {
        // read the last 2 bytes, and return the offset from nametable start
        // so 00, which means 0x2000, return 0, 01, which means 0x2400 return 0x0400 and so on
        match (self.get_bit(1), self.get_bit(0)) {
            (false, false) => 0x0000,
            (false, true) => 0x0400,
            (true, false) => 0x0800,
            (true, true) => 0x0c00,
        }
    }
}
