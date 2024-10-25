#[derive(Debug, Eq, PartialEq)]
pub struct PPUAddressReg {
    high_byte: u8,
    low_byte: u8,
    is_next_write_high: bool,
    last_byte: u8, // only to be able to simulate read
}

impl PPUAddressReg {
    pub fn new() -> Self {
        Self {
            high_byte: 0,
            low_byte: 0,
            is_next_write_high: true,
            last_byte: 0,
        }
    }

    pub fn get_address_as_u16(&self) -> u16 {
        ((self.high_byte as u16) << 8) | (self.low_byte as u16)
    }

    fn set_u16_address(&mut self, addr: u16) {
        self.high_byte = ((addr & 0x3fff) >> 8) as u8;
        self.low_byte = (addr & 0x00ff) as u8;
    }

    pub fn write_byte(&mut self, byte: u8) {
        if self.is_next_write_high {
            self.high_byte = byte & 0x3f; // when writing, bits 14 and 15 of the address are set to 0
        } else {
            self.low_byte = byte;
        }
        self.is_next_write_high = !self.is_next_write_high;
        self.last_byte = byte;
    }

    pub fn increment(&mut self, incr: u8) {
        self.set_u16_address(self.get_address_as_u16().wrapping_add(incr as u16));
    }

    pub fn reset_latch(&mut self) {
        self.is_next_write_high = true;
    }

    pub fn get_latch(&self) -> bool {
        self.is_next_write_high
    }

    pub fn read(&self) -> u8 {
        self.last_byte
    }
}
