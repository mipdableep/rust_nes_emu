pub struct PPUControlRegister {
    misc_settings: u8, // 7  bit  0
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
}

impl PPUControlRegister {
    pub fn new() -> Self {
        Self { misc_settings: 0 }
    }

    pub fn read(&self) -> u8 {
        self.misc_settings
    }

    pub fn write_byte(&mut self, value: u8) {
        self.misc_settings = value;
    }

    fn get_bit(&self, bit_location: u8) -> bool {
        // a function to return the value of the bit in bit location
        if bit_location > 7 {
            panic!("Error: Trying to access bit in location {bit_location} in the PPU control register, which does not exists");
        }
        (self.misc_settings >> bit_location) & 1 == 1
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
}
