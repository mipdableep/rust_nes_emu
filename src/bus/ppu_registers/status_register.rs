#[derive(Debug, Eq, PartialEq)]
pub struct PPUStatusRegister(u8);

impl PPUStatusRegister {
    // 7  bit  0
    // ---- ----
    // VSOx xxxx
    // |||| ||||
    // |||+-++++- (PPU open bus or 2C05 PPU identifier)
    // ||+------- Sprite overflow flag
    // |+-------- Sprite 0 hit flag
    // +--------- Vblank flag, cleared on read. Unreliable; see below.
    pub fn new() -> Self {
        Self(0)
    }

    pub fn read(&self) -> u8 {
        self.0
    }

    pub fn set_vblank_status(&mut self, new_status: bool) {
        match new_status {
            true => {
                self.0 |= 0x80;
            }
            false => {
                self.0 &= !0x80;
            }
        }
    }

    fn get_bit(&self, bit_location: u8) -> bool {
        // a function to return the value of the bit in bit location
        if bit_location > 7 {
            panic!("Error: Trying to access bit in location {bit_location} in the PPU control register, which does not exists");
        }
        (self.0 >> bit_location) & 1 == 1
    }

    pub fn get_vblank_status(&self) -> bool {
        self.get_bit(7)
    }

    pub fn set_sprite_0_hit_status(&mut self, new_status: bool) {
        match new_status {
            true => {
                self.0 |= 0x40;
            }
            false => {
                self.0 &= !0x40;
            }
        }
    }
}

#[test]
fn test_set_vblank_status() {
    let mut status_register = PPUStatusRegister::new();
    status_register.0 = 0;
    status_register.set_vblank_status(true);
    assert_eq!(status_register.read(), 0x80);
    status_register.0 = 0xFF;
    status_register.set_vblank_status(false);
    assert_eq!(status_register.read(), 0x7F);
}

#[test]
fn test_set_sprite_0_status() {
    let mut status_register = PPUStatusRegister::new();
    status_register.0 = 0;
    status_register.set_sprite_0_hit_status(true);
    assert_eq!(status_register.read(), 0b01000000);
    status_register.0 = 0xFF;
    status_register.set_sprite_0_hit_status(false);
    assert_eq!(status_register.read(), 0b10111111);
}
