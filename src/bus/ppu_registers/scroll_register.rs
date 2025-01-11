#[derive(Debug, Eq, PartialEq)]
pub struct PPUScrollReg {
    x_scroll: u8,
    y_scroll: u8,
    last_byte: u8, // only to be able to simulate read
                   // todo: still need to make the latch the same as the address register(?)
}

impl PPUScrollReg {
    pub fn new() -> Self {
        Self {
            x_scroll: 0,
            y_scroll: 0,
            last_byte: 0,
        }
    }

    pub fn write_byte(&mut self, byte: u8, is_next_write_x: &mut bool) {
        if *is_next_write_x {
            self.x_scroll = byte;
        } else {
            self.y_scroll = byte;
        }
        *is_next_write_x = !*is_next_write_x;
        self.last_byte = byte;
    }

    pub fn get_x_scroll(&self) -> u8 {
        self.x_scroll
    }

    pub fn get_y_scroll(&self) -> u8 {
        self.y_scroll
    }
}

#[test]
fn test_scroll_setting() {
    let mut scroll_reg = PPUScrollReg::new();
    let mut latch = true;
    assert_eq!(scroll_reg.get_x_scroll(), 0);
    assert_eq!(scroll_reg.get_y_scroll(), 0);

    scroll_reg.write_byte(0x10, &mut latch);
    scroll_reg.write_byte(0xab, &mut latch);
    scroll_reg.write_byte(0x20, &mut latch);

    assert_eq!(scroll_reg.get_x_scroll(), 0x20);
    assert_eq!(scroll_reg.get_y_scroll(), 0xab);
}
