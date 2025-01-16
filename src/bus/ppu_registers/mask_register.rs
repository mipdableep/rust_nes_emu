use std::ops::Deref;

#[derive(Debug, Eq, PartialEq)]
pub struct PPUMaskRegister(u8);
// 7  bit  0
// ---- ----
// BGRs bMmG
// |||| ||||
// |||| |||+- Greyscale (0: normal color, 1: greyscale)
// |||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
// |||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
// |||| +---- 1: Enable background rendering
// |||+------ 1: Enable sprite rendering
// ||+------- Emphasize red (green on PAL/Dendy)
// |+-------- Emphasize green (red on PAL/Dendy)
// +--------- Emphasize blue

impl Deref for PPUMaskRegister {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PPUMaskRegister {
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

    pub fn get_sprites_enabled(&self) -> bool {
        self.get_bit(4)
    }
}
