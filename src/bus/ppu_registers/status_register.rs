#[derive(Debug, Eq, PartialEq)]
pub struct PPUStatusRegister(u8);

impl PPUStatusRegister {
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
