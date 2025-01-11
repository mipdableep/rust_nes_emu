#[derive(Debug, Eq, PartialEq)]
pub struct PPUScrollReg {
    x_scroll: u8,
    y_scroll: u8,
    is_next_write_x: bool,
    last_byte: u8, // only to be able to simulate read
                   // todo: still need to make the latch the same as the address register(?)
}

impl PPUScrollReg {
    pub fn new() -> Self {
        Self {
            x_scroll: 0,
            y_scroll: 0,
            is_next_write_x: true,
            last_byte: 0,
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        if self.is_next_write_x {
            self.x_scroll = byte;
        } else {
            self.y_scroll = byte;
        }
        self.is_next_write_x = !self.is_next_write_x;
        self.last_byte = byte;
    }

    pub fn get_x_scroll(&self) -> u8 {
        self.x_scroll
    }

    pub fn get_y_scroll(&self) -> u8 {
        self.y_scroll
    }
}
