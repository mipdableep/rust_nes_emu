#[derive(Debug, Eq, PartialEq)]
pub struct ControllerByte {
    strobe: bool,
    read_numer: u8,
    value: u8,
    // 7  bit  0
    // ---- ----
    // RLDU SSBA
    // |||| ||||- A
    // |||| ||++- B
    // |||| |+--- Select
    // |||| +---- Start
    // |||+------ Up
    // ||+------- Down
    // |+-------- Left
    // +--------- Right
}

impl ControllerByte {
    pub fn new() -> ControllerByte {
        ControllerByte {
            strobe: false,
            read_numer: 0,
            value: 0,
        }
    }

    pub fn write(&mut self, value: u8) {
        // when writing, only the least significant bit is important
        // it turns the strobe on/off
        if value % 2 == 0 {
            self.strobe = false
        } else {
            self.read_numer = 0;
            self.strobe = true
        }
    }

    fn get_bit(&self, bit_location: u8) -> bool {
        // a function to return the value of the bit in bit location
        if bit_location > 7 {
            panic!("Error: Trying to access bit in location {bit_location} in the PPU control register, which does not exists");
        }
        (self.value >> bit_location) & 1 == 1
    }

    fn get_current_bit_status(&mut self) -> bool {
        let return_value = self.get_bit(self.read_numer);
        if !self.strobe && self.read_numer < 7 {
            self.read_numer += 1;
        }
        return_value
    }

    pub fn read(&mut self) -> u8 {
        // the controller should return 1 if the key is pressed and 9 if not
        // it should return a u8, since all reads work like this
        // so only the LSB is important
        // some games expect the value of the other bits to be equal to the last value written to the bus
        // which is usually 0x40 (the controllers are in 0x4016 and 0x4017, and little endian)
        if self.get_current_bit_status() {
            0x41
        } else {
            0x40
        }
    }

    pub fn set_right(&mut self) {
        self.value |= 0x80;
    }

    pub fn set_left(&mut self) {
        self.value |= 0x40;
    }

    pub fn set_down(&mut self) {
        self.value |= 0x20;
    }

    pub fn set_up(&mut self) {
        self.value |= 0x10;
    }

    pub fn set_start(&mut self) {
        self.value |= 0x08;
    }

    pub fn set_select(&mut self) {
        self.value |= 0x04;
    }

    pub fn set_b(&mut self) {
        self.value |= 0x02;
    }

    pub fn set_a(&mut self) {
        self.value |= 0x01;
    }

    pub fn unset_right(&mut self) {
        self.value &= !0x80;
    }

    pub fn unset_left(&mut self) {
        self.value &= !0x40;
    }

    pub fn unset_down(&mut self) {
        self.value &= !0x20;
    }

    pub fn unset_up(&mut self) {
        self.value &= !0x10;
    }

    pub fn unset_start(&mut self) {
        self.value &= !0x08;
    }

    pub fn unset_select(&mut self) {
        self.value &= !0x04;
    }

    pub fn unset_b(&mut self) {
        self.value &= !0x02;
    }

    pub fn unset_a(&mut self) {
        self.value &= !0x01;
    }
}
